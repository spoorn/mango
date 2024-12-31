use crate::core::mapped_registry::WritableRegistry;
use crate::core::registries::built_in_registries;
use crate::core::registries::built_in_registries::registry;
use crate::world::entity::entity_type;
use crate::world::item::items;
use crate::world::level::block::{blocks, composter_block, fire_block};
use crate::world::level::game_rules;
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

    // Should be impossible
    if built_in_registries::entity_type_registry()
        .get(entity_type::PLAYER.id)
        .is_none()
    {
        panic!("Failed loading EntityTypes");
    }

    game_rules::bootstrap();

    // TODO: EntitySelectorOptions, DispenseItemBehavior, CauldronInteraction
    // TODO: BuiltInRegistries.bootStrap() may not be necessary
    // TODO: CreativeModeTabs
}
