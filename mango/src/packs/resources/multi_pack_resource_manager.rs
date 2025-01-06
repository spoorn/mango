use crate::packs::pack_resources::PackResources;
use crate::packs::pack_type::PackType;
use crate::packs::resources::fallback_resource_manager::FallbackResourceManager;
use crate::packs::resources::resource_filter_section;
use crate::packs::resources::resource_filter_section::ResourceFilterSection;
use crate::resources::resource_location::ResourceLocation;
use itertools::Itertools;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug)]
pub struct MultiPackResourceManager {
    namespaced_managers: HashMap<String, FallbackResourceManager>,
    packs: Vec<Arc<dyn PackResources>>,
}
impl MultiPackResourceManager {
    pub fn new(pack_type: PackType, packs: Vec<Arc<dyn PackResources>>) -> Self {
        let namespaces: Vec<String> = packs
            .iter()
            .flat_map(|pr| pr.get_namespaces(pack_type))
            .unique()
            .collect();
        let mut namespaced_managers = HashMap::new();
        packs.iter().for_each(|pack| {
            let pack_filter_section = get_pack_filter_section(pack.clone()).map(Rc::new);
            let pack_namespaces = pack.get_namespaces(pack_type);
            let filter_predicate = || {
                pack_filter_section.clone().map(|section| {
                    Box::new(move |e: &ResourceLocation| section.is_path_filtered(&e.path)) as _
                })
            };

            namespaces.iter().for_each(|namespace| {
                let contains_namespace = pack_namespaces.contains(namespace);
                let is_namespace_filtered = pack_filter_section
                    .as_ref()
                    .map_or(false, |section| section.is_namespace_filtered(namespace));
                if contains_namespace || is_namespace_filtered {
                    let fallback_manager = namespaced_managers
                        .entry(namespace.clone())
                        .or_insert_with(|| {
                            FallbackResourceManager::new(pack_type, namespace.clone())
                        });

                    if contains_namespace && is_namespace_filtered {
                        fallback_manager.push_with_filter(Arc::clone(pack), filter_predicate());
                    } else if contains_namespace {
                        fallback_manager.push(Arc::clone(pack));
                    } else {
                        fallback_manager
                            .push_filter_only(pack.pack_id().clone(), filter_predicate());
                    }
                }
            });
        });

        Self {
            namespaced_managers,
            packs,
        }
    }
}

fn get_pack_filter_section(pack_resource: Arc<dyn PackResources>) -> Option<ResourceFilterSection> {
    pack_resource
        .get_metadata_section(resource_filter_section::TYPE)
        .map(|section| {
            section
                .as_any()
                .downcast_ref::<ResourceFilterSection>()
                .unwrap()
                .clone()
        })
}
