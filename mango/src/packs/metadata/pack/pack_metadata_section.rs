use crate::codec::Codec;
use crate::network::chat::mutable_component::MutableComponent;
use crate::packs::metadata::metadata_section_type::MetadataSectionType;
use crate::packs::metadata::pack::MetadataSection;
use serde_json::Value;
use std::ops::Range;

// Some serious Rust coercion magic: https://users.rust-lang.org/t/rule-s-about-casting-from-trait-implementation-type-to-dyn-trait-type/104392/4
pub const TYPE: MetadataSectionType = MetadataSectionType::new("pack", |e| {
    PackMetadataSection::decode_boxed(e).map(|e| e as _)
});

#[derive(Debug)]
pub struct PackMetadataSection {
    description: MutableComponent,
    pack_format: u32,
    supported_formats: Option<Range<u32>>,
}
impl PackMetadataSection {
    pub fn new(
        description: MutableComponent,
        pack_format: u32,
        supported_formats: Option<Range<u32>>,
    ) -> Self {
        Self {
            description,
            pack_format,
            supported_formats,
        }
    }
}
impl MetadataSection for PackMetadataSection {}
impl Codec for PackMetadataSection {
    type Data = Value;

    fn decode(data: Self::Data) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}
