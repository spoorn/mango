use crate::packs::repository::pack::Pack;
use crate::packs::repository::repository_source::RepositorySource;
use std::collections::HashMap;
use tracing::info;

#[derive(Debug)]
pub struct PackRepository {
    sources: Vec<Box<dyn RepositorySource>>,
    available: HashMap<String, Pack>,
    selected: Vec<Pack>,
}
impl PackRepository {
    pub fn new(sources: impl Iterator<Item = Box<dyn RepositorySource>>) -> Self {
        Self {
            sources: sources.collect(),
            available: HashMap::new(),
            selected: Vec::new(),
        }
    }

    pub fn reload(&mut self) {
        let curr = self
            .selected
            .iter()
            .map(|p| p.location.id.clone())
            .collect();
        self.available = self.discover_available();
        self.selected = self.rebuild_selected(curr);
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

    fn rebuild_selected(&self, ids: Vec<String>) -> Vec<Pack> {
        let mut available_packs: Vec<Pack> = ids
            .iter()
            .filter_map(|id| self.available.get(id))
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
}
