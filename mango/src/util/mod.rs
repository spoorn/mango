use crate::resources::resource_location::ResourceLocation;
use regex::Regex;
use serde::{Deserialize, Deserializer};

pub mod datafix;
pub mod directory_lock;
pub mod mth;
pub mod resource_location_pattern;

pub fn make_description_id(id: &str, location: ResourceLocation) -> String {
    format!(
        "{}.{}.{}",
        id,
        location.namespace,
        location.path.replace("/", ".")
    )
}

fn deserialize_regex<'de, D>(deserializer: D) -> Result<Regex, D::Error>
where
    D: Deserializer<'de>,
{
    Regex::new(&String::deserialize(deserializer)?)
        .map_err(|e| serde::de::Error::custom(format!("Failed to deserialize regex: {:?}", e)))
}
