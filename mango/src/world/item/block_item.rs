use crate::core::Indexed;
use crate::world::item::item::{ItemTrait, Properties};
use crate::world::level::block::block::BlockTrait;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BlockItem {
    block: usize,
}
#[typetag::serialize]
impl ItemTrait for BlockItem {
    fn is_block_item(&self) -> bool {
        true
    }
}
impl BlockItem {
    pub fn new<T: BlockTrait>(block: Indexed<T>, properties: Properties) -> Self {
        Self { block: block.id }
    }
    pub fn register_blocks() {
        // register blocks
    }
}
