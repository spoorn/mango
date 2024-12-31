use crate::network::chat::mutable_component::MutableComponent;
use crate::packs::metadata::pack::MetadataSection;
use std::ops::Range;

pub const TYPE: &str = "pack";

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
