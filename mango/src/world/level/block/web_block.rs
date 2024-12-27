use crate::world::level::block::block::{Block, BlockTrait};
use crate::world::level::block::state::block_behavior::{BlockBehaviour, Properties};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct WebBlock {
    block: Block,
}

impl BlockBehaviour for WebBlock {
    fn get_block(&self) -> &Block {
        &self.block
    }
}

#[typetag::serialize]
impl BlockTrait for WebBlock {}
impl WebBlock {
    pub fn new(properties: Properties) -> Self {
        Self {
            block: Block::new(properties),
        }
    }
}
