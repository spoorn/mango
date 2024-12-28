use crate::world::level::storage::data_version::DataVersion;
use serde::Deserialize;

// TODO: Add default if version.json is missing
#[derive(Debug, Deserialize)]
pub struct WorldVersion {
    pub id: String,
    pub name: String,
    #[serde(flatten)]
    pub world_version: DataVersion,
    pub protocol_version: u32,
    pub pack_version: PackVersion,
    pub build_time: String,
    pub java_component: String,
    pub stable: bool,
    pub use_editor: bool,
}

#[derive(Debug, Deserialize)]
pub struct PackVersion {
    pub resource: u32,
    pub data: u32,
}

pub fn detect_version() -> WorldVersion {
    serde_json::from_slice(include_bytes!("../../resources/version.json"))
        .expect("failed to parse world version")
}
