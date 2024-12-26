use crate::world::level::block::block::BlockTrait;
use crate::world::level::block::state::block_behavior;

#[derive(Debug)]
pub struct FireBlock {}

impl FireBlock {
    pub fn new(properties: block_behavior::Properties) -> Self {
        Self {}
    }
}

impl BlockTrait for FireBlock {}
