use crate::core::Indexed;
use crate::world::item::item::ItemTrait;
use crate::world::item::items;
use crate::world::level::block::block::{Block, BlockTrait};
use crate::world::level::block::state::block_behavior::{BlockBehaviour, Properties};
use dashmap::DashMap;
use serde::Serialize;
use std::sync::LazyLock;
use tracing::info;

/// Item Id to compost value
pub static COMPOSTABLES: LazyLock<DashMap<usize, f32>> = LazyLock::new(|| DashMap::new());

pub fn bootstrap() {
    add(0.3, items::JUNGLE_LEAVES.get().unwrap());
    info!("Loaded compostables: {:#?}", COMPOSTABLES);
}

fn add(value: f32, item: &Indexed<impl ItemTrait>) {
    COMPOSTABLES.insert(item.id, value);
}

#[derive(Debug, Serialize)]
pub struct ComposterBlock {
    block: Block,
}
impl BlockBehaviour for ComposterBlock {
    fn get_block(&self) -> &Block {
        &self.block
    }
}
#[typetag::serialize]
impl BlockTrait for ComposterBlock {}
impl ComposterBlock {
    pub fn new(properties: Properties) -> Self {
        Self {
            block: Block::new(properties),
        }
    }
}
