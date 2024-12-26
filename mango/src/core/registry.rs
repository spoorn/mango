use crate::core::registration_info;
use crate::core::registries::built_in_registries::WritableRegistry;
use crate::resources::resource_key::ResourceKey;
use std::sync::Arc;

pub fn register_key<T, R: WritableRegistry<Arc<T>>>(
    mut registry: Arc<R>,
    key: ResourceKey,
    value: Arc<T>,
) -> Arc<T> {
    registry.register(key, Arc::clone(&value), registration_info::BUILT_IN);
    value
}
