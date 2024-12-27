use std::ops::Deref;
use std::sync::Arc;

pub mod block_pos;
pub mod direction;
pub mod mapped_registry;
pub mod registration_info;
pub mod registries;
pub mod registry;
pub mod vec3i;

pub struct Indexed<T> {
    pub id: usize,
    pub value: Arc<T>,
}
impl<T> Deref for Indexed<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<T> Clone for Indexed<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            value: Arc::clone(&self.value),
        }
    }
}
