use crate::packs::metadata::metadata_section_type::MetadataSectionType;
use crate::packs::metadata::pack::MetadataSection;
use serde_json::Value;
use std::io::{BufReader, Read};

/// This is synchronous right now. May want to make it async if this gets called from multiple
/// sources. For now it's fine because the stream should be loaded into the binary instead of
/// reading from a file on the system and is only called on startup and reload commands.
pub fn get_metadata_from_stream(
    metadata_section_type: &MetadataSectionType,
    stream: impl Read,
) -> Option<Box<dyn MetadataSection>> {
    let reader = BufReader::new(stream);
    let mut json: Value = serde_json::from_reader(reader).ok()?;
    match json.get_mut(metadata_section_type.name) {
        None => None,
        Some(json) => (metadata_section_type.codec)(json.take()).ok(),
    }
}
