use crate::codec::Codec;
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

    pub fn read(value: impl AsRef<str>) -> ResourceLocation {
        Self::by_separator(value, ':')
    }

    // Note: this follows vanilla but seems to allow invalid ResourceLocations, like empty string
    pub fn by_separator(value: impl AsRef<str>, separator: char) -> ResourceLocation {
        let value = value.as_ref();
        match value.find(separator) {
            None => Self::with_default_namespace(&value),
            Some(index) => {
                let right = &value[(index + 1)..];
                if index != 0 {
                    let left = &value[..index];
                    Self::new(left.to_string(), right.to_string())
                } else {
                    Self::with_default_namespace(right)
                }
            }
        }
    }
}
impl Display for ResourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}
// TODO: waiting for https://github.com/rust-lang/rust/issues/63063 to avoid cloning Strings
// For now we lie and claim this is a static str as it *should* live long enough
impl Codec<&'static str> for ResourceLocation {
    fn decode(data: &'static str) -> anyhow::Result<Self> {
        Ok(Self::read(data))
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
