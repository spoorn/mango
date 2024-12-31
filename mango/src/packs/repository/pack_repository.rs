use crate::packs::repository::pack::Pack;
use crate::packs::repository::repository_source::RepositorySource;
use std::collections::HashMap;

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
}
