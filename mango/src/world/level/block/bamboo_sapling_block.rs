use crate::world::level::block::block::{Block, BlockTrait};
use crate::world::level::block::state::block_behavior::{BlockBehaviour, Properties};
use std::ops::Deref;

#[derive(Debug)]
pub struct BambooSaplingBlock {
    block: Block,
}

impl BlockBehaviour for BambooSaplingBlock {
    fn get_block(&self) -> &Block {
        &self.block
    }
}

impl BlockTrait for BambooSaplingBlock {}
impl Deref for BambooSaplingBlock {
    type Target = Block;

    fn deref(&self) -> &Self::Target {
        &self.block
    }
}
impl BambooSaplingBlock {
    pub fn new(properties: Properties) -> Self {
        Self {
            block: Block::new(properties),
        }
    }
}
