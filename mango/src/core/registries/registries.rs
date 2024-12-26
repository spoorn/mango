use crate::resources::resource_key::ResourceKey;
use crate::resources::resource_location::ResourceLocation;
use std::sync::LazyLock;

pub static BLOCK: LazyLock<ResourceKey> = LazyLock::new(|| {
    create_registry_key(ResourceLocation::with_default_namespace(
        "block".to_string(),
    ))
});

pub fn root_registry_name() -> ResourceLocation {
    ResourceLocation::with_default_namespace("root".to_string())
}

pub fn create_registry_key(location: ResourceLocation) -> ResourceKey {
    ResourceKey::create_registry_key(location)
}
