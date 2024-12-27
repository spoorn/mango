use crate::world::phys::shapes::bit_set_discrete_voxel_shape::BitSetDiscreteVoxelShape;
use crate::world::phys::shapes::discrete_voxel_shape::DiscreteVoxelShape;
use std::any::Any;

pub trait VoxelShapeTrait {
    fn as_any(&self) -> &dyn Any;
    fn equals(&self, other: &dyn VoxelShapeTrait) -> bool;
}
impl<S: PartialEq + 'static> VoxelShapeTrait for S {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn equals(&self, other: &dyn VoxelShapeTrait) -> bool {
        other
            .as_any()
            .downcast_ref::<S>()
            .map_or(false, |a| self == a)
    }
}

// pub trait VoxelShapeTraitComparable: VoxelShapeTrait + PartialEq {}
// impl<T: VoxelShapeTrait + PartialEq> VoxelShapeTraitComparable for T {}
#[derive(Eq, PartialEq)]
pub struct VoxelShape {
    pub shape: DiscreteVoxelShape,
}
// impl VoxelShapeTrait for Box<dyn VoxelShapeTrait> {}
impl VoxelShape {
    pub fn new(discrete_voxel_shape: DiscreteVoxelShape) -> Self {
        Self {
            shape: discrete_voxel_shape,
        }
    }
}
impl From<BitSetDiscreteVoxelShape> for VoxelShape {
    fn from(value: BitSetDiscreteVoxelShape) -> Self {
        Self::new(value.discrete_voxel_shape)
    }
}
