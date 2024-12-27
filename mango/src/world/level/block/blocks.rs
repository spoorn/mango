use crate::core::block_pos::BlockPos;
use crate::core::registries::{built_in_registries, registries};
use crate::core::{registry, Indexed};
use crate::resources::resource_key::ResourceKey;
use crate::resources::resource_location::ResourceLocation;
use crate::world::entity::entity_type;
use crate::world::level::block::bamboo_sapling_block::BambooSaplingBlock;
use crate::world::level::block::block::BlockTrait;
use crate::world::level::block::fire_block::FireBlock;
use crate::world::level::block::sound_type;
use crate::world::level::block::sound_type::SoundType;
use crate::world::level::block::state::block_behavior::{BlockState, OffsetType, Properties};
use crate::world::level::block::web_block::WebBlock;
use crate::world::level::block_getter::BlockGetter;
use crate::world::level::material::map_color;
use crate::world::level::material::push_reaction::PushReaction;
use std::sync::{Arc, OnceLock};

pub static FIRE: OnceLock<Indexed<Arc<FireBlock>>> = OnceLock::new();
pub static COBWEB: OnceLock<Indexed<Arc<WebBlock>>> = OnceLock::new();
pub static BAMBOO_SAPLING: OnceLock<Indexed<Arc<BambooSaplingBlock>>> = OnceLock::new();

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
                .light_level(|_| 15)
                .sound_type(sound_type::WOOL.clone())
                .push_reaction(PushReaction::Destroy)
                .build(),
        )
    });
    COBWEB.get_or_init(|| {
        register_block(
            "cobweb",
            WebBlock::new,
            Properties::builder()
                .map_color(|_| map_color::WOOL)
                .no_collision()
                .force_solid_on(true)
                .sound_type(sound_type::COBWEB.clone())
                .requires_correct_tool_for_drops(true)
                .strength(4.0)
                .push_reaction(PushReaction::Destroy)
                .build(),
        )
    });
    BAMBOO_SAPLING.get_or_init(|| {
        register_block(
            "bamboo_sapling",
            BambooSaplingBlock::new,
            Properties::builder()
                .map_color(|_| map_color::WOOD)
                .force_solid_on(true)
                .random_ticks()
                .no_collision()
                // Note: vanilla has a bug here where it also sets instabreak
                .strength(1.0)
                .sound_type(sound_type::BAMBOO_SAPLING.clone())
                .offset_type(OffsetType::XZ)
                .ignited_by_lava(true)
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

fn leaves_properties(sound_type: SoundType) -> Properties {
    Properties::builder()
        .map_color(|_| map_color::PLANT)
        .strength(0.2)
        .random_ticks()
        .sound_type(sound_type)
        .no_occlusion()
        .is_valid_spawn(ocelot_or_parrot)
        .is_suffocating(never)
        .is_view_blocking(never)
        .ignited_by_lava(true)
        .push_reaction(PushReaction::Destroy)
        .is_redstone_conductor(never)
        .build()
}

fn never(_block_state: &BlockState, _block_getter: &dyn BlockGetter, _block_pos: BlockPos) -> bool {
    false
}

fn ocelot_or_parrot(
    _block_state: &BlockState,
    _block_getter: &dyn BlockGetter,
    _block_pos: BlockPos,
    entity_type: usize,
) -> bool {
    entity_type == entity_type::OCELOT.get().unwrap().id
        || entity_type == entity_type::PARROT.get().unwrap().id
}
