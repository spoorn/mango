use crate::core::vec3i::Vec3i;
use std::ops::Deref;

pub struct BlockPos {
    vec3i: Vec3i,
}
impl Deref for BlockPos {
    type Target = Vec3i;

    fn deref(&self) -> &Self::Target {
        &self.vec3i
    }
}
