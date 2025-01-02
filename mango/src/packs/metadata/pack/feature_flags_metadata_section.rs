use crate::codec::Codec;
use crate::packs::metadata::metadata_section_type::MetadataSectionType;
use crate::packs::metadata::pack::MetadataSection;
use crate::world::flag::feature_flag_set::FeatureFlagSet;
use serde_json::Value;
use std::any::Any;

const TYPE_NAME: &str = "features";
// Some serious Rust coercion magic: https://users.rust-lang.org/t/rule-s-about-casting-from-trait-implementation-type-to-dyn-trait-type/104392/4
pub const TYPE: MetadataSectionType = MetadataSectionType::new(TYPE_NAME, |e| {
    FeatureFlagsMetadataSection::decode_boxed(e).map(|e| e as _)
});

#[derive(Debug)]
pub struct FeatureFlagsMetadataSection {
    pub flags: FeatureFlagSet,
}
impl MetadataSection for FeatureFlagsMetadataSection {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Codec<Value> for FeatureFlagsMetadataSection {
    fn decode(data: Value) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        FeatureFlagSet::decode(data).map(|flags| Self { flags })
    }
}
