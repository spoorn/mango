use crate::network::chat::mutable_component::MutableComponent;
use crate::packs::built_in_metadata::BuiltInMetadata;
use crate::packs::metadata::pack::feature_flags_metadata_section::FeatureFlagsMetadataSection;
use crate::packs::metadata::pack::pack_metadata_section::PackMetadataSection;
use crate::packs::metadata::pack::{
    feature_flags_metadata_section, pack_metadata_section, MetadataSection,
};
use crate::packs::pack_location_info::PackLocationInfo;
use crate::packs::pack_resources::PackResources;
use crate::packs::pack_selection_config::PackSelectionConfig;
use crate::packs::pack_type::PackType;
use crate::packs::repository::folder_repository_source;
use crate::packs::repository::folder_repository_source::FolderRepositorySource;
use crate::packs::repository::known_pack::KnownPack;
use crate::packs::repository::pack::{Metadata, Pack, Position, ResourcesSupplier};
use crate::packs::repository::pack_repository::PackRepository;
use crate::packs::repository::pack_source::PackSource;
use crate::packs::repository::repository_source::RepositorySource;
use crate::packs::vanilla_pack_resources::{VanillaPackResources, VanillaPackResourcesBuilder};
use crate::resources::resource_location::ResourceLocation;
use crate::shared_constants;
use crate::world::flag::feature_flags;
use crate::world::level::storage::level_resource::LevelResource;
use crate::world::level::storage::level_storage_source::{LevelStorageAccess, LevelStorageSource};
use crate::world::level::validation::directory_validator::DirectoryValidator;
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use tracing::warn;

const VANILLA_SELECTION_CONFIG: PackSelectionConfig = PackSelectionConfig {
    required: false,
    default_position: Position::Bottom,
    fixed_position: false,
};
const FEATURE_SELECTION_CONFIG: PackSelectionConfig = PackSelectionConfig {
    required: false,
    default_position: Position::Top,
    fixed_position: false,
};

#[derive(Debug)]
pub struct ServerPacksSource {
    pack_type: PackType,
    vanilla_pack: VanillaPackResources,
    pack_dir: ResourceLocation,
    validator: DirectoryValidator,
}
impl ServerPacksSource {
    pub fn new(validator: DirectoryValidator) -> Self {
        Self {
            pack_type: PackType::ServerData,
            vanilla_pack: create_vanilla_pack_source(),
            pack_dir: packs_dir(),
            validator,
        }
    }

    fn list_bundled_packs(&self, consumer: &mut dyn FnMut(Pack) -> ()) {
        let mut suppliers = HashMap::new();
        self.populate_pack_list(|id, supplier| {
            suppliers.insert(id, supplier);
        });
        suppliers.into_iter().for_each(|(id, supplier)| {
            if let Some(pack) = supplier(id) {
                consumer(pack);
            }
        });
    }

    fn populate_pack_list(
        &self,
        mut consumer: impl FnMut(String, Box<dyn FnOnce(String) -> Option<Pack>>) -> (),
    ) {
        self.vanilla_pack
            .list_raw_paths(self.pack_type, &self.pack_dir, |dir_entry| {
                self.discover_packs_in_path(dir_entry, &mut consumer)
            });
    }

    /// Consumer unfortunately needs to Box the nested pointer due to closures capturing variables
    /// unable to be coerced into function pointers.
    fn discover_packs_in_path(
        &self,
        path: Option<&'static include_dir::DirEntry>,
        consumer: &mut impl FnMut(String, Box<dyn FnOnce(String) -> Option<Pack>>) -> (),
    ) {
        match path {
            None => warn!(
                "Failed to discover pack type {} in path: {:?}",
                self.pack_type, self.pack_dir
            ),
            Some(entry) => match entry.as_dir() {
                None => warn!(
                    "Pack type {} in path {:?} is not a directory",
                    self.pack_type, self.pack_dir
                ),
                Some(dir) => {
                    folder_repository_source::discover_packs(
                        dir,
                        self.validator,
                        |path, supplier| {
                            consumer(
                                Self::path_to_id(path.path()),
                                Box::new(move |id| {
                                    let component = get_pack_title(id.as_str());
                                    create_built_in_pack(id, supplier, component)
                                }),
                            )
                        },
                    );
                }
            },
        }
    }

    /// wtf
    fn path_to_id(path: &Path) -> String {
        let mut path = path.file_name().unwrap().to_str().unwrap();
        if let Some(stripped_suffix) = path.strip_suffix(".zip") {
            path = stripped_suffix;
        }
        path.to_string()
    }
}
impl RepositorySource for ServerPacksSource {
    fn load_packs(&self, consumer: &mut dyn FnMut(Pack) -> ()) {
        if let Some(pack) = create_vanilla_pack(self.vanilla_pack.clone()) {
            consumer(pack);
        }
        self.list_bundled_packs(consumer);
    }
}

#[derive(Debug)]
struct FixedResources<T: PackResources + Debug + 'static>(Arc<T>);
impl<T: PackResources + Debug + 'static> ResourcesSupplier for FixedResources<T> {
    fn open_primary(&self, _location: &PackLocationInfo) -> Arc<dyn PackResources> {
        Arc::clone(&self.0) as _
    }

    fn open_full(
        &self,
        _location: &PackLocationInfo,
        _metadata: &Metadata,
    ) -> Arc<dyn PackResources> {
        Arc::clone(&self.0) as _
    }
}

fn built_in_metadata() -> BuiltInMetadata {
    BuiltInMetadata::of(
        [
            (
                pack_metadata_section::TYPE,
                Rc::new(PackMetadataSection::new(
                    MutableComponent::translatable("dataPack.vanilla.description"),
                    shared_constants::WORLD_VERSION.get_pack_version(PackType::ServerData),
                    None,
                )) as Rc<dyn MetadataSection>,
            ),
            (
                feature_flags_metadata_section::TYPE,
                Rc::new(FeatureFlagsMetadataSection {
                    flags: feature_flags::FEATURE_FLAGS.default_flags.clone(),
                }) as Rc<dyn MetadataSection>,
            ),
        ]
        .into_iter(),
    )
}

fn create_vanilla_pack_source() -> VanillaPackResources {
    VanillaPackResourcesBuilder::default()
        .set_metadata(built_in_metadata())
        .expose_namespace(["minecraft".to_string()].into_iter())
        .apply_development_config()
        .push_jar_resources()
        .build(vanilla_pack_info())
}

fn packs_dir() -> ResourceLocation {
    ResourceLocation::with_default_namespace("datapacks")
}

fn core_pack_info() -> KnownPack {
    KnownPack::vanilla("core")
}

fn vanilla_pack_info() -> PackLocationInfo {
    PackLocationInfo::new(
        "vanilla".to_string(),
        MutableComponent::translatable("dataPack.vanilla.name"),
        PackSource::BuiltIn,
        Some(core_pack_info()),
    )
}

fn create_built_in_pack_location(id: String, component: MutableComponent) -> PackLocationInfo {
    PackLocationInfo::new(id, component, PackSource::Feature, Some(core_pack_info()))
}

fn get_pack_title(id: &str) -> MutableComponent {
    MutableComponent::literal(id)
}

fn create_vanilla_pack(pack_resource: impl PackResources + Debug + 'static) -> Option<Pack> {
    Pack::read_meta_and_create(
        vanilla_pack_info(),
        Rc::new(FixedResources(Arc::new(pack_resource))),
        PackType::ServerData,
        VANILLA_SELECTION_CONFIG,
    )
}

fn create_built_in_pack(
    id: String,
    supplier: Rc<dyn ResourcesSupplier>,
    component: MutableComponent,
) -> Option<Pack> {
    Pack::read_meta_and_create(
        create_built_in_pack_location(id, component),
        supplier,
        PackType::ServerData,
        FEATURE_SELECTION_CONFIG,
    )
}

pub fn create_pack_repository(
    source: &LevelStorageSource,
    access: &mut LevelStorageAccess,
) -> PackRepository {
    let path = access.get_level_path(LevelResource::DatapackDir);
    PackRepository::new(
        [
            // Built in packs
            Box::new(ServerPacksSource::new(source.world_dir_validator))
                as Box<dyn RepositorySource>,
            // External datapacks
            Box::new(FolderRepositorySource::new(
                path,
                PackType::ServerData,
                PackSource::World,
                source.world_dir_validator,
            )),
        ]
        .into_iter(),
    )
}
