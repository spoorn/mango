use crate::world::level::block::block::{Block, BlockTrait};
use crate::world::level::block::state::block_behavior::{BlockBehaviour, Properties};

#[derive(Debug)]
pub struct ComposterBlock {
    block: Block,
}
impl BlockBehaviour for ComposterBlock {
    fn get_block(&self) -> &Block {
        &self.block
    }
}
impl BlockTrait for ComposterBlock {}
impl ComposterBlock {
    pub fn new(properties: Properties) -> Self {
        Self {
            block: Block::new(properties),
        }
    }
}
