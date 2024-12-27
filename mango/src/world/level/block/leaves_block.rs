use crate::world::level::block::block::{Block, BlockTrait};
use crate::world::level::block::state::block_behavior::{BlockBehaviour, Properties};

#[derive(Debug)]
pub struct LeavesBlock {
    block: Block,
}

impl BlockBehaviour for LeavesBlock {
    fn get_block(&self) -> &Block {
        &self.block
    }
}

impl BlockTrait for LeavesBlock {}
impl LeavesBlock {
    pub fn new(properties: Properties) -> Self {
        Self {
            block: Block::new(properties),
        }
    }
}
