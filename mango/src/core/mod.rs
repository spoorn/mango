use std::ops::Deref;
use std::sync::{Arc, LazyLock};

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

/// Global Indexed
pub struct GlobalIndexed<T, F = fn() -> Indexed<T>>(LazyLock<Indexed<T>, F>);
impl<T, F: FnOnce() -> Indexed<T>> GlobalIndexed<T, F> {
    pub const fn new(f: F) -> Self {
        Self(LazyLock::new(f))
    }

    pub fn init(&self) -> &Indexed<T> {
        &*self.0
    }
}
impl<T> Deref for GlobalIndexed<T> {
    type Target = Indexed<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Global<T, F = fn() -> T>(LazyLock<T, F>);
impl<T, F: FnOnce() -> T> Global<T, F> {
    pub const fn new(f: F) -> Self {
        Self(LazyLock::new(f))
    }

    pub fn init(&self) -> &T {
        &*self.0
    }
}
impl<T> Deref for Global<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
