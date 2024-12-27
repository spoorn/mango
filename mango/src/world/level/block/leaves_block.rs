use crate::world::level::block::block::{Block, BlockTrait};
use crate::world::level::block::state::block_behavior::{BlockBehaviour, Properties};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LeavesBlock {
    block: Block,
}

impl BlockBehaviour for LeavesBlock {
    fn get_block(&self) -> &Block {
        &self.block
    }
}

#[typetag::serialize]
impl BlockTrait for LeavesBlock {}
impl LeavesBlock {
    pub fn new(properties: Properties) -> Self {
        Self {
            block: Block::new(properties),
        }
    }
}
