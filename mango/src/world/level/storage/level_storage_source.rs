use crate::codec::Codec;
use crate::nbt::compound_tag::CompoundTag;
use crate::nbt::nbt_accounter::NbtAccounter;
use crate::nbt::{nbt_io, nbt_ops, nbt_utils};
use crate::util::datafix::data_fix_types::DataFixTypes;
use crate::util::datafix::data_fixers;
use crate::util::datafix::data_fixers::DataFixer;
use crate::util::datafix::serialization::dynamic::Dynamic;
use crate::util::directory_lock::DirectoryLock;
use crate::world::flag::feature_flags;
use crate::world::level::level_settings::LevelSettings;
use crate::world::level::storage::level_resource::LevelResource;
use crate::world::level::storage::level_summary::LevelSummary;
use crate::world::level::storage::level_version::LevelVersion;
use crate::world::level::validation::directory_validator::DirectoryValidator;
use crate::world::level::world_data_configuration::WorldDataConfiguration;
use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::error;

const ALLOWED_SYMLINKS_FILE: &str = "allowed_symlinks.txt";

pub struct LevelStorageSource {
    base_dir: PathBuf,
    backup_dir: PathBuf,
    fixer_upper: Arc<DataFixer>,
    pub world_dir_validator: DirectoryValidator,
}
impl LevelStorageSource {
    pub fn new(
        base_dir: PathBuf,
        backup_dir: PathBuf,
        dir_validator: DirectoryValidator,
        data_fixer: Arc<DataFixer>,
    ) -> Self {
        if let Err(e) = std::fs::create_dir_all(&base_dir) {
            panic!("Failed to create base directory for level: {:?}", e);
        }

        Self {
            base_dir,
            backup_dir,
            fixer_upper: data_fixer,
            world_dir_validator: dir_validator,
        }
    }

    pub fn get_data_fixer(&self) -> Arc<DataFixer> {
        Arc::clone(&self.fixer_upper)
    }

    pub fn create_default(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let directory_validator = parse_validator(path.join(ALLOWED_SYMLINKS_FILE));
        let backups = path.join("../backups");
        Self::new(
            path,
            backups,
            directory_validator,
            data_fixers::get_data_fixer(),
        )
    }

    pub fn validate_and_create_access(&self, level_name: String) -> LevelStorageAccess {
        let level_path = self.base_dir.join(&level_name);
        let errors = self
            .world_dir_validator
            .validate_directory(level_path.clone(), true);
        if !errors.is_empty() {
            panic!("Level directory validation failed: {:#?}", errors);
        }
        LevelStorageAccess::new(level_name, level_path)
    }
}

fn parse_validator(path: PathBuf) -> DirectoryValidator {
    if std::fs::exists(&path).unwrap_or_else(|_| panic!("Failed to check if {:?} exists", path)) {
        match File::open(&path) {
            Ok(_file) => {
                todo!("{} is not yet supported", ALLOWED_SYMLINKS_FILE);
            }
            Err(e) => {
                error!(
                    ?e,
                    ?path,
                    "Failed to parse {}, disallowing all symbolic links",
                    ALLOWED_SYMLINKS_FILE
                );
            }
        }
    }

    DirectoryValidator::new(|_| false)
}

async fn read_level_data_tag_fixed(
    path: PathBuf,
    data_fixer: impl AsRef<DataFixer>,
) -> Result<Dynamic<CompoundTag>> {
    let root = read_level_data_tag_raw(path).await?;
    let data = root.get_compound("Data");
    let data_version = nbt_utils::get_data_version(&data, 0);
    let res = DataFixTypes::Level.update_to_current_version(
        data_fixer.as_ref(),
        Dynamic::new(nbt_ops::INSTANCE.clone(), data),
        data_version,
    );
    // TODO: DFU for Player and WorldGenSettings. Vanilla is a mess with this
    Ok(res)
}

async fn read_level_data_tag_raw(path: PathBuf) -> Result<CompoundTag> {
    nbt_io::read_compressed(path, NbtAccounter::create(104857600)).await
}

pub struct LevelDirectory {
    path: PathBuf,
}
impl LevelDirectory {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn directory_name(&self) -> String {
        self.path.file_name().unwrap().to_str().unwrap().to_string()
    }

    pub fn data_file(&self) -> PathBuf {
        self.resource_path(LevelResource::LevelDataFile)
    }

    pub fn old_data_file(&self) -> PathBuf {
        self.resource_path(LevelResource::OldLevelDataFile)
    }

    fn resource_path(&self, level_resource: LevelResource) -> PathBuf {
        self.path.join(level_resource.id())
    }

    pub fn icon_file(&self) -> PathBuf {
        self.resource_path(LevelResource::IconFile)
    }
}

pub struct LevelStorageAccess {
    lock: DirectoryLock,
    pub level_directory: LevelDirectory,
    level_id: String,
    resources: HashMap<LevelResource, PathBuf>,
}
impl LevelStorageAccess {
    pub fn new(level_id: String, level_directory: PathBuf) -> Self {
        Self {
            lock: DirectoryLock::create(level_directory.clone()),
            level_directory: LevelDirectory::new(level_directory),
            level_id,
            resources: HashMap::new(),
        }
    }

    pub fn get_level_path(&mut self, level_resource: LevelResource) -> PathBuf {
        self.resources
            .entry(level_resource)
            .or_insert_with(|| self.level_directory.resource_path(level_resource))
            .clone()
    }

    pub fn has_world_data(&self) -> bool {
        self.level_directory.data_file().exists() || self.level_directory.old_data_file().exists()
    }

    // We differ from vanilla in that this must be for a CompoundTag
    // TODO: move this out to share with LevelSummary readLevelSummary
    pub fn make_level_summary(&self, dynamic: &Dynamic<CompoundTag>, locked: bool) -> LevelSummary {
        let level_version = LevelVersion::parse(dynamic);
        let level_data_version = level_version.level_data_version;
        if level_data_version != 19132 && level_data_version != 19133 {
            panic!("Unknown data version: {}", level_data_version);
        }
        let is_not_storage_version = level_data_version != 19133;
        let icon_file = self.level_directory.icon_file();
        let world_data_configuration = read_data_config(dynamic);
        let level_settings = LevelSettings::parse(dynamic, world_data_configuration);
        let is_experimental =
            feature_flags::is_experimental(&level_settings.data_configuration.enabled_features);
        LevelSummary::new(
            level_settings,
            level_version,
            // TODO: This differs from vanilla but I believe this should be the same thing
            self.level_id.clone(),
            is_not_storage_version,
            locked,
            is_experimental,
            icon_file,
        )
    }

    pub async fn get_data_tag(
        &self,
        use_old_data_file: bool,
        data_fixer: impl AsRef<DataFixer>,
    ) -> Result<Dynamic<CompoundTag>> {
        // No need to check lock here as DirectoryLock only unlocks on drop
        read_level_data_tag_fixed(
            if use_old_data_file {
                self.level_directory.old_data_file()
            } else {
                self.level_directory.data_file()
            },
            data_fixer,
        )
        .await
    }

    pub fn restore_level_data_from_old(&self) -> bool {
        todo!("Restoring level data from old is not yet supported. This means the level data is corrupted but the old level data is valid.");
    }
}

pub fn read_data_config(dynamic: &Dynamic<CompoundTag>) -> WorldDataConfiguration {
    WorldDataConfiguration::decode(dynamic.clone()).unwrap_or_default()
}
