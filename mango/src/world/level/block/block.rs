use crate::world::level::block::state::block_behavior::{BlockBehaviour, Properties};
use crate::world::phys::shapes::shapes;
use crate::world::phys::shapes::voxel_shape::VoxelShapeTrait;
use std::borrow::Borrow;
use std::fmt::Debug;

pub trait BlockTrait: BlockBehaviour + Send + Sync + Debug {}

#[derive(Debug)]
pub struct Block {
    properties: Properties,
}

impl BlockBehaviour for Block {
    fn get_block(&self) -> &Block {
        self
    }
}

impl BlockTrait for Block {}
impl Block {
    pub fn new(properties: Properties) -> Self {
        Self { properties }
    }
}

// TODO: cache this
pub fn is_shape_full_block(shape: &dyn VoxelShapeTrait) -> bool {
    !shapes::join_is_not_empty(shapes::block().borrow(), shape)
}
