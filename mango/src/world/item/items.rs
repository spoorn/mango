use crate::core::registries::built_in_registries;
use crate::core::{registries::registries, registry, Indexed};
use crate::resources::resource_key::ResourceKey;
use crate::world::item::block_item::BlockItem;
use crate::world::item::item::{ItemTrait, Properties, PropertiesBuilder};
use crate::world::level::block::block::BlockTrait;
use std::sync::Arc;

fn block_id_to_item_id(block_key: &ResourceKey) -> ResourceKey {
    ResourceKey::create(&registries::ITEM, block_key.location.clone())
}

pub fn register_block<T: BlockTrait>(block: Indexed<Arc<T>>) -> Indexed<Arc<BlockItem>> {
    register_block_item_with_fn(block, BlockItem::new)
}

pub fn register_block_item_with_fn<T: BlockTrait, O: ItemTrait + 'static>(
    block: Indexed<Arc<T>>,
    item_fn: fn(Indexed<Arc<T>>, Properties) -> O,
) -> Indexed<Arc<O>> {
    register_block_item_with_properties(block, item_fn, Properties::builder())
}

pub fn register_block_item_with_properties<T: BlockTrait, O: ItemTrait + 'static>(
    block: Indexed<Arc<T>>,
    item_fn: fn(Indexed<Arc<T>>, Properties) -> O,
    properties: PropertiesBuilder,
) -> Indexed<Arc<O>> {
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
) -> Indexed<Arc<T>> {
    properties.id = Some(key.clone());
    let item = Arc::new(item_fn(properties));
    // TODO: Use BlockItem::register_blocks
    registry::register_key(built_in_registries::item_registry(), key, item)
}
