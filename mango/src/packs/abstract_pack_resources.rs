use crate::packs::metadata::metadata_section_type::MetadataSectionType;
use crate::packs::metadata::pack::MetadataSection;
use serde_json::Value;
use std::io::{BufReader, Read};
use tracing::info;

/// Deserializes pack.mcmeta into its sections.
///
/// See https://minecraft.wiki/w/Pack.mcmeta for details on the format.
///
/// We only support reading the `pack` and `features` fields for now on the server side.
///
/// This is synchronous right now. May want to make it async if this gets called from multiple
/// sources. For now I think it's fine because this should only be called on startup and reload
/// commands.
pub fn get_metadata_from_stream(
    metadata_section_type: &MetadataSectionType,
    stream: impl Read,
) -> Option<Box<dyn MetadataSection>> {
    let reader = BufReader::new(stream);
    let mut json: Value = serde_json::from_reader(reader).ok()?;
    match json.get_mut(metadata_section_type.name) {
        None => {
            info!(
                "Missing metadata section: {} in JSON: {:?}",
                metadata_section_type.name, json
            );
            None
        }
        Some(json) => (metadata_section_type.codec)(json.take())
            .inspect_err(|e| {
                panic!(
                    "Failed to deserialize a metadata section {:?}: {:?}",
                    metadata_section_type, e
                );
            })
            .ok(),
    }
}
