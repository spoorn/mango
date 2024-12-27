use serde::Serialize;

pub const ZERO: Vec3 = Vec3::new(0.0, 0.0, 0.0);

#[derive(Debug, Serialize)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}
