use crate::world::phys::shapes::array_voxel_shape::ArrayVoxelShape;
use crate::world::phys::shapes::bit_set_discrete_voxel_shape::BitSetDiscreteVoxelShape;
use crate::world::phys::shapes::discrete_voxel_shape::DiscreteVoxelShape;
use crate::world::phys::shapes::voxel_shape::{VoxelShape, VoxelShapeTrait};

pub fn empty() -> Box<dyn VoxelShapeTrait> {
    Box::new(ArrayVoxelShape::new(
        DiscreteVoxelShape::new(0, 0, 0),
        vec![0.0],
        vec![0.0],
        vec![0.0],
    ))
}

pub fn block() -> Box<dyn VoxelShapeTrait> {
    // TODO: Not yet sure if this is necessary instead of just creating the DiscreteVoxelShape directly
    let mut bit_set_discrete_voxel_shape = BitSetDiscreteVoxelShape::new(1, 1, 1);
    bit_set_discrete_voxel_shape.fill(0, 0, 0);
    Box::new(VoxelShape::from(bit_set_discrete_voxel_shape))
}

// TODO: implement
pub fn join_is_not_empty(s1: &dyn VoxelShapeTrait, s2: &dyn VoxelShapeTrait) -> bool {
    s1.equals(s2)
}
