use crate::util::datafix::serialization::dynamic_ops::DynamicOps;
use std::sync::Arc;

// TODO: Can we avoid this and do better? It's way too messy in vanilla code
#[derive(Clone)]
pub struct Dynamic<T: Clone> {
    ops: Arc<dyn DynamicOps>,
    pub value: T,
}
impl<T: Clone> Dynamic<T> {
    pub fn new(ops: Arc<dyn DynamicOps>, value: T) -> Self {
        Self { ops, value }
    }

    pub fn update(self, key: &str, update_fn: impl FnOnce(T) -> T) -> Self {
        todo!();
    }
}
