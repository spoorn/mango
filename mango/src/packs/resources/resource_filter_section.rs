use crate::codec::Codec;
use crate::packs::metadata::metadata_section_type::MetadataSectionType;
use crate::packs::metadata::pack::MetadataSection;
use crate::util::resource_location_pattern::ResourceLocationPattern;
use serde::Deserialize;
use serde_json::Value;
use std::any::Any;

const TYPE_NAME: &str = "filter";
// Some serious Rust coercion magic: https://users.rust-lang.org/t/rule-s-about-casting-from-trait-implementation-type-to-dyn-trait-type/104392/4
pub const TYPE: MetadataSectionType = MetadataSectionType::new(TYPE_NAME, |e| {
    ResourceFilterSection::decode_boxed(e).map(|e| e as _)
});

#[derive(Clone, Debug, Deserialize)]
pub struct ResourceFilterSection {
    #[serde(rename = "block")]
    block_list: Vec<ResourceLocationPattern>,
}
impl ResourceFilterSection {
    pub fn is_namespace_filtered(&self, namespace: &str) -> bool {
        self.block_list
            .iter()
            .any(|pattern| pattern.namespace_predicate(namespace))
    }

    pub fn is_path_filtered(&self, path: &str) -> bool {
        self.block_list
            .iter()
            .any(|pattern| pattern.path_predicate(path))
    }
}
impl MetadataSection for ResourceFilterSection {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Codec<Value> for ResourceFilterSection {
    fn decode(data: Value) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(serde_json::from_value(data)?)
    }
}
