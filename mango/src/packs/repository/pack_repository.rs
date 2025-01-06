use crate::packs::pack_resources::PackResources;
use crate::packs::repository::pack::Pack;
use crate::packs::repository::repository_source::RepositorySource;
use crate::world::flag::feature_flag_set::FeatureFlagSet;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;

#[derive(Debug)]
pub struct PackRepository {
    sources: Vec<Box<dyn RepositorySource>>,
    pub available: HashMap<String, Pack>,
    pub selected: Vec<Pack>,
}
impl PackRepository {
    pub fn new(sources: impl Iterator<Item = Box<dyn RepositorySource>>) -> Self {
        Self {
            sources: sources.collect(),
            available: HashMap::new(),
            selected: Vec::new(),
        }
    }

    pub fn set_selected(&mut self, selected: &[&String]) {
        self.selected = self.rebuild_selected(selected);
    }

    pub fn get_selected_ids(&self) -> Vec<&String> {
        self.selected.iter().map(|p| &p.location.id).collect()
    }

    pub fn is_available(&self, id: &String) -> bool {
        self.available.contains_key(id)
    }

    pub fn reload(&mut self) {
        let curr: Vec<&String> = self.selected.iter().map(|p| &p.location.id).collect();
        self.available = self.discover_available();
        self.selected = self.rebuild_selected(&curr);
    }

    fn discover_available(&self) -> HashMap<String, Pack> {
        let mut available = HashMap::new();
        self.sources.iter().for_each(|source| {
            source.load_packs(&mut |pack| {
                info!("Discovered pack\n{}", pack);
                available.insert(pack.location.id.clone(), pack);
            });
        });
        available
    }

    fn rebuild_selected(&self, ids: &[&String]) -> Vec<Pack> {
        let mut available_packs: Vec<Pack> = ids
            .iter()
            .filter_map(|id| self.available.get(*id))
            .cloned()
            .collect();
        self.available.values().for_each(|pack| {
            if pack.is_required() && !available_packs.contains(&pack) {
                pack.selection_config
                    .default_position
                    .insert(&mut available_packs, pack.clone());
            }
        });
        available_packs
    }

    pub fn get_requested_feature_flags(&self) -> FeatureFlagSet {
        self.selected
            .iter()
            .map(Pack::get_requested_features)
            .cloned()
            .reduce(FeatureFlagSet::join)
            .unwrap_or_else(|| FeatureFlagSet::empty())
    }

    pub fn open_all_selected(&self) -> Vec<Arc<dyn PackResources>> {
        self.selected.iter().map(Pack::open).collect()
    }
}
