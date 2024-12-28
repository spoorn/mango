use crate::dedicated::dedicated_server_properties::DedicatedServerProperties;
use std::path::PathBuf;

#[derive(Debug)]
pub struct DedicatedServerSettings {
    path: PathBuf,
    properties: DedicatedServerProperties,
}
impl DedicatedServerSettings {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path: PathBuf = path.into();
        Self {
            path: path.clone(),
            properties: DedicatedServerProperties::from_file(path),
        }
    }

    pub fn force_save(&self) {
        self.properties.store(&self.path);
    }
}
