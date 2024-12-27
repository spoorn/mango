use crate::world::phys::shapes::discrete_voxel_shape::DiscreteVoxelShape;

// TODO: Add BitSet
pub struct BitSetDiscreteVoxelShape {
    pub discrete_voxel_shape: DiscreteVoxelShape,
    x_min: u32,
    y_min: u32,
    z_min: u32,
    x_max: u32,
    y_max: u32,
    z_max: u32,
}
impl BitSetDiscreteVoxelShape {
    pub fn new(x_min: u32, y_min: u32, z_min: u32) -> Self {
        Self {
            discrete_voxel_shape: DiscreteVoxelShape::new(x_min, y_min, z_min),
            x_min,
            y_min,
            z_min,
            x_max: 0,
            y_max: 0,
            z_max: 0,
        }
    }

    pub fn fill(&mut self, x_min: u32, y_min: u32, z_min: u32) {
        self.fill_update_bounds(x_min, y_min, z_min, true);
    }

    fn fill_update_bounds(&mut self, x_min: u32, y_min: u32, z_min: u32, set_local_bounds: bool) {
        if set_local_bounds {
            self.x_min = u32::min(self.x_min, x_min);
            self.y_min = u32::min(self.y_min, y_min);
            self.z_min = u32::min(self.z_min, z_min);
            self.x_max = u32::max(self.x_max, x_min + 1);
            self.y_max = u32::max(self.y_max, y_min + 1);
            self.z_max = u32::max(self.z_max, z_min + 1);
        }
    }
}
