use crate::shared_constants;
use std::fmt::Display;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KnownPack {
    pub namespace: String,
    pub id: String,
    pub version: String,
}
impl KnownPack {
    pub fn vanilla(id: &str) -> Self {
        Self {
            namespace: "minecraft".to_string(),
            id: id.to_string(),
            version: shared_constants::WORLD_VERSION.id.clone(),
        }
    }
}
impl Display for KnownPack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.namespace, self.id, self.version)
    }
}
