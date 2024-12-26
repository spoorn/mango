use crate::core::registries::{built_in_registries, registries};
use crate::core::{registry, Indexed};
use crate::resources::resource_key::ResourceKey;
use crate::resources::resource_location::ResourceLocation;
use crate::world::level::block::block::BlockTrait;
use crate::world::level::block::fire_block::FireBlock;
use crate::world::level::block::sound_type;
use crate::world::level::block::state::block_behavior::Properties;
use crate::world::level::material::map_color;
use crate::world::level::material::push_reaction::PushReaction;
use std::sync::{Arc, OnceLock};

pub static FIRE: OnceLock<Indexed<Arc<FireBlock>>> = OnceLock::new();

pub fn bootstrap() {
    FIRE.get_or_init(|| {
        register_block(
            "fire",
            FireBlock::new,
            Properties::builder()
                .map_color(|_| map_color::FIRE)
                .replaceable(true)
                .no_collision()
                .instabreak()
                .sound_type(sound_type::WOOL.clone())
                .push_reaction(PushReaction::Destroy)
                .build(),
        )
    });
}

pub fn vanilla_block_id(path: &str) -> ResourceKey {
    ResourceKey::create(
        &registries::BLOCK,
        ResourceLocation::with_default_namespace(path),
    )
}

fn register_block<T: BlockTrait + 'static>(
    path: &str,
    block_fn: fn(Properties) -> T,
    properties: Properties,
) -> Indexed<Arc<T>> {
    register(vanilla_block_id(path), block_fn, properties)
}

fn register<T: BlockTrait + 'static>(
    key: ResourceKey,
    block_fn: fn(Properties) -> T,
    mut properties: Properties,
) -> Indexed<Arc<T>> {
    properties.id = Some(key.clone());
    let block = Arc::new(block_fn(properties));
    registry::register_key(built_in_registries::block_registry(), key, block)
}
