use crate::core::registries::built_in_registries;
use crate::core::{registries::registries, registry, GlobalIndexed, Indexed};
use crate::resources::resource_key::ResourceKey;
use crate::world::item::block_item::BlockItem;
use crate::world::item::item::{ItemTrait, Properties, PropertiesBuilder};
use crate::world::level::block::block::BlockTrait;
use crate::world::level::block::blocks;
use std::sync::Arc;

pub static JUNGLE_LEAVES: GlobalIndexed<BlockItem> =
    GlobalIndexed::new(|| register_block(blocks::JUNGLE_LEAVES.clone()));

pub fn bootstrap() {
    JUNGLE_LEAVES.init();
}

fn block_id_to_item_id(block_key: &ResourceKey) -> ResourceKey {
    ResourceKey::create(&registries::ITEM, block_key.location.clone())
}

pub fn register_block<T: BlockTrait>(block: Indexed<T>) -> Indexed<BlockItem> {
    register_block_item_with_fn(block, BlockItem::new)
}

pub fn register_block_item_with_fn<T: BlockTrait, O: ItemTrait + 'static>(
    block: Indexed<T>,
    item_fn: fn(Indexed<T>, Properties) -> O,
) -> Indexed<O> {
    register_block_item_with_properties(block, item_fn, Properties::builder())
}

pub fn register_block_item_with_properties<T: BlockTrait, O: ItemTrait + 'static>(
    block: Indexed<T>,
    item_fn: fn(Indexed<T>, Properties) -> O,
    properties: PropertiesBuilder,
) -> Indexed<O> {
    register(
        block_id_to_item_id(&built_in_registries::block_registry().key),
        |prop| item_fn(block, prop),
        properties.use_block_description_prefix().build(),
    )
}

pub fn register<T: ItemTrait + 'static>(
    key: ResourceKey,
    item_fn: impl FnOnce(Properties) -> T,
    mut properties: Properties,
) -> Indexed<T> {
    properties.id = Some(key.clone());
    let item = Arc::new(item_fn(properties));
    // TODO: Use BlockItem::register_blocks
    registry::register_key(built_in_registries::item_registry(), key, item)
}
