use crate::codec::Codec;
use crate::network::chat::mutable_component::MutableComponent;
use crate::packs::metadata::metadata_section_type::MetadataSectionType;
use crate::packs::metadata::pack::MetadataSection;
use serde::Deserialize;
use serde_json::Value;
use std::any::Any;
use std::ops::RangeInclusive;

const TYPE_NAME: &str = "pack";
// Some serious Rust coercion magic: https://users.rust-lang.org/t/rule-s-about-casting-from-trait-implementation-type-to-dyn-trait-type/104392/4
pub const TYPE: MetadataSectionType = MetadataSectionType::new(TYPE_NAME, |e| {
    PackMetadataSection::decode_boxed(e).map(|e| e as _)
});

#[derive(Debug, Deserialize)]
pub struct PackMetadataSection {
    #[serde(deserialize_with = "MutableComponent::deserialize")]
    pub description: MutableComponent,
    pub pack_format: u32,
    pub supported_formats: Option<RangeInclusive<u32>>,
}
impl PackMetadataSection {
    pub fn new(
        description: MutableComponent,
        pack_format: u32,
        supported_formats: Option<RangeInclusive<u32>>,
    ) -> Self {
        Self {
            description,
            pack_format,
            supported_formats,
        }
    }
}
impl MetadataSection for PackMetadataSection {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Codec<Value> for PackMetadataSection {
    fn decode(data: Value) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(serde_json::from_value(data)?)
    }
}
