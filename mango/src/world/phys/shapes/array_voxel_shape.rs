use crate::world::phys::shapes::discrete_voxel_shape::DiscreteVoxelShape;
use crate::world::phys::shapes::voxel_shape::VoxelShape;

#[derive(PartialEq)]
pub struct ArrayVoxelShape {
    voxel_shape: VoxelShape,
    xs: Vec<f64>,
    ys: Vec<f64>,
    zs: Vec<f64>,
}
impl ArrayVoxelShape {
    pub fn new(
        discrete_voxel_shape: DiscreteVoxelShape,
        xs: Vec<f64>,
        ys: Vec<f64>,
        zs: Vec<f64>,
    ) -> Self {
        if discrete_voxel_shape.x_size + 1 != xs.len() as u32
            || discrete_voxel_shape.y_size + 1 != ys.len() as u32
            || discrete_voxel_shape.z_size + 1 != zs.len() as u32
        {
            panic!("Lengths of point arrays must be consistent with the size of the VoxelShape.");
        }
        Self {
            voxel_shape: VoxelShape::new(discrete_voxel_shape),
            xs,
            ys,
            zs,
        }
    }
}
