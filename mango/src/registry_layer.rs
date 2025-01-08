use crate::core::layered_registry_access::LayeredRegistryAccess;
use crate::core::registries::built_in_registries;
use crate::core::registry_access::RegistryAccess;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Copy, Clone, Debug, EnumIter, PartialEq, Eq)]
pub enum RegistryLayer {
    Static,
    WorldGen,
    Dimensions,
    Reloadable,
}

pub fn create_registry_access() -> LayeredRegistryAccess<RegistryLayer> {
    LayeredRegistryAccess::new(RegistryLayer::iter().collect()).replace_from(
        RegistryLayer::Static,
        vec![RegistryAccess::from_registry_of_registries(
            built_in_registries::registry(),
        )],
    )
}
