#[derive(Eq, PartialEq)]
pub struct DiscreteVoxelShape {
    pub x_size: u32,
    pub y_size: u32,
    pub z_size: u32,
}
impl DiscreteVoxelShape {
    pub fn new(x_size: u32, y_size: u32, z_size: u32) -> Self {
        Self {
            x_size,
            y_size,
            z_size,
        }
    }
}
