use crate::world::level::block::block::{Block, BlockTrait};
use crate::world::level::block::blocks;
use crate::world::level::block::state::block_behavior::{BlockBehaviour, Properties};
use dashmap::DashMap;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FireBlock {
    block: Block,
    pub ignite_odds: DashMap<usize, u32>,
    pub burn_odds: DashMap<usize, u32>,
}

impl BlockBehaviour for FireBlock {
    fn get_block(&self) -> &Block {
        &self.block
    }
}

#[typetag::serialize]
impl BlockTrait for FireBlock {}

impl FireBlock {
    pub fn new(properties: Properties) -> Self {
        Self {
            block: Block::new(properties),
            ignite_odds: DashMap::new(),
            burn_odds: DashMap::new(),
        }
    }

    pub fn set_flammable(&self, block_id: usize, ignite_odds: u32, burn_odds: u32) {
        self.ignite_odds.insert(block_id, ignite_odds);
        self.burn_odds.insert(block_id, burn_odds);
    }
}

pub fn bootstrap() {
    let fire_block = blocks::FIRE.get().unwrap();
    fire_block.set_flammable(1, 5, 20);
}
