use crate::util;
use regex::Regex;
use serde::{Deserialize, Deserializer};

#[derive(Clone, Debug, Deserialize)]
pub struct ResourceLocationPattern {
    #[serde(deserialize_with = "util::deserialize_regex")]
    namespace_pattern: Regex,
    #[serde(deserialize_with = "util::deserialize_regex")]
    path_pattern: Regex,
}
impl ResourceLocationPattern {
    pub fn namespace_predicate(&self, namespace: &str) -> bool {
        self.namespace_pattern.is_match(namespace)
    }

    pub fn path_predicate(&self, path: &str) -> bool {
        self.path_pattern.is_match(path)
    }
}
