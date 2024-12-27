use serde_with::SerializeDisplay;
use std::fmt::Display;

#[derive(Clone, Debug, Eq, Hash, PartialEq, SerializeDisplay)]
pub struct ResourceLocation {
    pub namespace: String,
    pub path: String,
}

impl ResourceLocation {
    pub fn new(namespace: String, path: String) -> Self {
        assert!(
            is_valid_namespace(&namespace),
            "Invalid namespace: {}",
            namespace
        );
        assert!(is_valid_path(&path), "Invalid path: {}", path);
        Self { namespace, path }
    }

    pub fn with_default_namespace(path: &str) -> Self {
        Self::new("minecraft".to_string(), path.to_string())
    }
}

impl Display for ResourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

fn is_valid_namespace(namespace: &str) -> bool {
    namespace
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-' || c == '.')
}

fn is_valid_path(path: &str) -> bool {
    path.chars().all(|c| {
        c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-' || c == '.' || c == '/'
    })
}
