use crate::core::Indexed;
use crate::world::item::item::{ItemTrait, Properties};
use crate::world::level::block::block::BlockTrait;
use std::sync::Arc;

#[derive(Debug)]
pub struct BlockItem {
    block: usize,
}
impl ItemTrait for BlockItem {
    fn is_block_item(&self) -> bool {
        true
    }
}
impl BlockItem {
    pub fn new<T: BlockTrait>(block: Indexed<Arc<T>>, properties: Properties) -> Self {
        Self { block: block.id }
    }
    pub fn register_blocks() {
        // register blocks
    }
}
