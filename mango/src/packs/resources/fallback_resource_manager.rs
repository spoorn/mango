use crate::packs::pack_resources::PackResources;
use crate::packs::pack_type::PackType;
use crate::packs::resources::resource_manager::ResourceManager;
use crate::resources::resource_location::ResourceLocation;
use std::sync::Arc;

pub struct FallbackResourceManager {
    fallbacks: Vec<PackEntry>,
    pack_type: PackType,
    namespace: String,
}
impl FallbackResourceManager {
    pub fn new(pack_type: PackType, namespace: String) -> Self {
        Self {
            fallbacks: Vec::new(),
            pack_type,
            namespace,
        }
    }

    pub fn push(&mut self, resource: Arc<dyn PackResources>) {
        self.push_internal(resource.pack_id().clone(), Some(resource), None);
    }

    pub fn push_with_filter(
        &mut self,
        resource: Arc<dyn PackResources>,
        filter: Option<Box<dyn Fn(&ResourceLocation) -> bool>>,
    ) {
        self.push_internal(resource.pack_id().clone(), Some(resource), filter);
    }

    pub fn push_filter_only(
        &mut self,
        pack_id: String,
        filter: Option<Box<dyn Fn(&ResourceLocation) -> bool>>,
    ) {
        self.push_internal(pack_id, None, filter);
    }

    fn push_internal(
        &mut self,
        name: String,
        resources: Option<Arc<dyn PackResources>>,
        filter: Option<Box<dyn Fn(&ResourceLocation) -> bool>>,
    ) {
        self.fallbacks.push(PackEntry {
            name,
            resources,
            filter,
        });
    }
}
impl ResourceManager for FallbackResourceManager {}

struct PackEntry {
    name: String,
    resources: Option<Arc<dyn PackResources>>,
    filter: Option<Box<dyn Fn(&ResourceLocation) -> bool>>,
}
