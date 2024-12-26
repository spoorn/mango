use crate::core::registration_info;
use crate::core::registries::built_in_registries::WritableRegistry;
use crate::resources::resource_key::ResourceKey;
use crate::resources::resource_location::ResourceLocation;
use std::sync::Arc;

pub fn register_key<T, R: WritableRegistry<Arc<T>>>(
    mut registry: Arc<R>,
    key: ResourceKey,
    value: Arc<T>,
) -> Arc<T> {
    registry.register(key, Arc::clone(&value), registration_info::BUILT_IN);
    value
}

pub fn register_location<T, R: WritableRegistry<Arc<T>>>(
    mut registry: Arc<R>,
    location: ResourceLocation,
    value: Arc<T>,
) -> Arc<T> {
    let key = ResourceKey::create(registry.key(), location);
    register_key(registry, key, value)
}

pub fn register_key_take<T, R: WritableRegistry<T>>(
    mut registry: Arc<R>,
    key: ResourceKey,
    value: T,
) {
    registry.register(key, value, registration_info::BUILT_IN);
}

pub fn register_location_take<T, R: WritableRegistry<T>>(
    mut registry: Arc<R>,
    location: ResourceLocation,
    value: T,
) {
    let key = ResourceKey::create(registry.key(), location);
    register_key_take(registry, key, value);
}
