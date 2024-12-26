use crate::core::registries::built_in_registries;
use crate::core::registries::built_in_registries::registry;
use crate::world::level::block::blocks;
use tracing::info;

pub fn bootstrap() {
    built_in_registries::bootstrap();
    blocks::bootstrap();
    let registry = registry();
    info!("Loaded registries: {:#?}", registry);
    if registry.is_key_set_empty() {
        panic!("Unable to load registries");
    }
}
