use crate::core::registries::registries::root_registry_name;
use crate::resources::resource_location::ResourceLocation;
use std::fmt::{Display, Formatter};

// TODO: Cache

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ResourceKey {
    pub registry_name: ResourceLocation,
    pub location: ResourceLocation,
}

impl ResourceKey {
    pub fn new(registry_name: ResourceLocation, location: ResourceLocation) -> Self {
        Self {
            registry_name,
            location,
        }
    }

    pub fn create(key: &ResourceKey, location: ResourceLocation) -> Self {
        Self::new(key.location.clone(), location)
    }

    pub fn create_registry_key(location: ResourceLocation) -> Self {
        Self::new(root_registry_name(), location)
    }
}

impl Display for ResourceKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResourceKey[{} / {}]", self.registry_name, self.location)
    }
}

impl Default for ResourceKey {
    fn default() -> Self {
        todo!()
    }
}
