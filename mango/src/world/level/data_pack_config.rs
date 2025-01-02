use crate::codec::Codec;
use crate::nbt::compound_tag::CompoundTag;
use crate::nbt::tag::Tag;
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DataPackConfig {
    pub enabled: Vec<String>,
    pub disabled: Vec<String>,
}
impl DataPackConfig {
    pub const fn new(enabled: Vec<String>, disabled: Vec<String>) -> Self {
        Self { enabled, disabled }
    }
}
impl Default for DataPackConfig {
    fn default() -> Self {
        Self {
            enabled: vec!["vanilla".to_string()],
            disabled: Vec::new(),
        }
    }
}
impl Codec<CompoundTag> for DataPackConfig {
    fn decode(data: CompoundTag) -> Result<Self> {
        let enabled = data
            .get_list("Enabled")
            .iter()
            .filter_map(|tag| match tag {
                Tag::StringTag(s) => Some(s),
                _ => None,
            })
            .cloned()
            .collect::<Vec<String>>();

        let disabled = data
            .get_list("Disabled")
            .iter()
            .filter_map(|tag| match tag {
                Tag::StringTag(s) => Some(s),
                _ => None,
            })
            .cloned()
            .collect();

        Ok(Self { enabled, disabled })
    }
}
