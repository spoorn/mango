use crate::packs::pack_type::PackType;
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
impl WorldVersion {
    pub fn get_pack_version(&self, pack_type: PackType) -> u32 {
        match pack_type {
            PackType::ClientResources => self.pack_version.resource,
            PackType::ServerData => self.pack_version.data,
        }
    }
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
