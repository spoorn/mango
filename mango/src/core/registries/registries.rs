use crate::resources::resource_key::ResourceKey;
use crate::resources::resource_location::ResourceLocation;
use std::sync::LazyLock;

pub static BLOCK: LazyLock<ResourceKey> = LazyLock::new(|| create_default_registry_key("block"));
pub static ITEM: LazyLock<ResourceKey> = LazyLock::new(|| create_default_registry_key("item"));
pub static ENTITY_TYPE: LazyLock<ResourceKey> =
    LazyLock::new(|| create_default_registry_key("entity_type"));
pub static SOUND_EVENT: LazyLock<ResourceKey> =
    LazyLock::new(|| create_default_registry_key("sound_event"));

pub fn root_registry_name() -> ResourceLocation {
    ResourceLocation::with_default_namespace("root")
}

pub fn create_registry_key(location: ResourceLocation) -> ResourceKey {
    ResourceKey::create_registry_key(location)
}

pub fn create_default_registry_key(path: &str) -> ResourceKey {
    create_registry_key(ResourceLocation::with_default_namespace(path))
}
