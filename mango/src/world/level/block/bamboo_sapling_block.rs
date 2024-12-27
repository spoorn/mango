use crate::world::level::block::block::{Block, BlockTrait};
use crate::world::level::block::state::block_behavior::{BlockBehaviour, Properties};
use serde::Serialize;
use std::ops::Deref;

#[derive(Debug, Serialize)]
pub struct BambooSaplingBlock {
    block: Block,
}

impl BlockBehaviour for BambooSaplingBlock {
    fn get_block(&self) -> &Block {
        &self.block
    }
}

#[typetag::serialize]
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
