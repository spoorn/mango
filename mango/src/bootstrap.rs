use crate::core::registries::built_in_registries;
use crate::core::registries::built_in_registries::registry;
use crate::world::entity::entity_type;
use crate::world::item::items;
use crate::world::level::block::{blocks, composter_block, fire_block};
use tracing::info;

pub fn bootstrap() {
    built_in_registries::bootstrap();
    blocks::bootstrap();
    items::bootstrap();
    entity_type::bootstrap();
    let registry = registry();
    if registry.is_key_set_empty() {
        panic!("Unable to load registries");
    }
    fire_block::bootstrap();
    composter_block::bootstrap();
    info!(
        "Loaded registries: {}",
        serde_json::to_string_pretty(&registry).unwrap()
    );
}
