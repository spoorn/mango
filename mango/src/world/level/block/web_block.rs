use crate::world::level::block::block::{Block, BlockTrait};
use crate::world::level::block::state::block_behavior::{BlockBehaviour, Properties};

#[derive(Debug)]
pub struct WebBlock {
    block: Block,
}

impl BlockBehaviour for WebBlock {
    fn get_block(&self) -> &Block {
        &self.block
    }
}

impl BlockTrait for WebBlock {}
impl WebBlock {
    pub fn new(properties: Properties) -> Self {
        Self {
            block: Block::new(properties),
        }
    }
}
