use crate::util::datafix::serialization::dynamic_ops::DynamicOps;
use std::sync::{Arc, LazyLock};

pub const INSTANCE: LazyLock<Arc<dyn DynamicOps>> = LazyLock::new(|| Arc::new(NbtOps {}));

pub struct NbtOps {}
impl DynamicOps for NbtOps {}
