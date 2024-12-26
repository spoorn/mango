use std::fmt::Display;

pub struct KnownPack {
    pub namespace: String,
    pub id: String,
    pub version: String,
}

impl Display for KnownPack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.namespace, self.id, self.version)
    }
}
