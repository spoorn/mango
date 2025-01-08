use crate::core::mapped_registry::{MappedRegistry, Registry, WritableRegistry};
use crate::core::registries::built_in_registries;
use crate::resources::resource_key::ResourceKey;
use dashmap::DashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct RegistryAccess {
    pub registries: DashMap<ResourceKey, Arc<MappedRegistry<Arc<dyn Registry>>>>,
}
impl RegistryAccess {
    pub fn new(registries: DashMap<ResourceKey, Arc<MappedRegistry<Arc<dyn Registry>>>>) -> Self {
        Self { registries }
    }

    pub fn empty() -> Self {
        Self {
            registries: DashMap::new(),
        }
    }

    pub fn from_registry_of_registries(registry: Arc<MappedRegistry<Arc<dyn Registry>>>) -> Self {
        Self {
            registries: DashMap::from_iter([(registry.key.clone(), registry)]),
        }
    }

    pub fn lookup(&self, key: &ResourceKey) -> Option<Arc<dyn Registry>> {
        // TODO: this is simplified from vanilla to avoid creating another trait by assuming the
        // RegistryAccess with a single registry is the root registry only
        if self.registries.len() == 1 {
            self.registries
                .get(built_in_registries::registry().key())
                .expect("Expected registry access with single registry to be the root registry")
                .get_optional_by_key(key)
        } else {
            self.registries.get(key).map(|e| Arc::clone(&e) as _)
        }
    }
}
