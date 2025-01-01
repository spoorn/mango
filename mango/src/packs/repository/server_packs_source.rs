use crate::network::chat::mutable_component::MutableComponent;
use crate::packs::built_in_metadata::BuiltInMetadata;
use crate::packs::metadata::pack::feature_flags_metadata_section::FeatureFlagsMetadataSection;
use crate::packs::metadata::pack::pack_metadata_section::PackMetadataSection;
use crate::packs::metadata::pack::{
    feature_flags_metadata_section, pack_metadata_section, MetadataSection,
};
use crate::packs::pack_location_info::PackLocationInfo;
use crate::packs::pack_resources::PackResources;
use crate::packs::pack_type::PackType;
use crate::packs::repository::folder_repository_source::FolderRepositorySource;
use crate::packs::repository::known_pack::KnownPack;
use crate::packs::repository::pack::{Metadata, Pack, ResourcesSupplier};
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
use std::rc::Rc;

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
}
impl RepositorySource for ServerPacksSource {
    fn load_packs(&self, consumer: &dyn FnOnce(Pack) -> ()) {
        todo!();
    }
}

struct FixedResources<T: PackResources>(T);
impl<T: PackResources> ResourcesSupplier<T> for FixedResources<T> {
    fn open_primary(&self, _location: PackLocationInfo) -> &T {
        &self.0
    }

    fn open_full(&self, _location: PackLocationInfo, _metadata: Metadata) -> &T {
        &self.0
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

pub fn create_pack_repository(
    source: &LevelStorageSource,
    access: &mut LevelStorageAccess,
) -> PackRepository {
    let path = access.get_level_path(LevelResource::DatapackDir);
    PackRepository::new(
        [
            Box::new(ServerPacksSource::new(source.world_dir_validator))
                as Box<dyn RepositorySource>,
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
