use crate::world::level::block::block::BlockTrait;
use crate::world::level::block::blocks;
use crate::world::level::block::state::block_behavior;
use dashmap::DashMap;

#[derive(Debug)]
pub struct FireBlock {
    pub ignite_odds: DashMap<usize, u32>,
    pub burn_odds: DashMap<usize, u32>,
}

impl FireBlock {
    pub fn new(properties: block_behavior::Properties) -> Self {
        Self {
            ignite_odds: DashMap::new(),
            burn_odds: DashMap::new(),
        }
    }

    pub fn set_flammable(&self, block_id: usize, ignite_odds: u32, burn_odds: u32) {
        self.ignite_odds.insert(block_id, ignite_odds);
        self.burn_odds.insert(block_id, burn_odds);
    }
}

impl BlockTrait for FireBlock {}

pub fn bootstrap() {
    let fire_block = blocks::FIRE.get().unwrap();
    fire_block.set_flammable(1, 5, 20);
}
