use encoding_rs::UTF_8;
use jiff::Zoned;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use tracing::{error, warn};

// TODO: implement all properties
#[derive(Debug, Serialize, Deserialize)]
pub struct DedicatedServerProperties {
    #[serde(rename = "server-ip")]
    server_ip: String,
    #[serde(rename = "server-port")]
    server_port: u16,
}
impl DedicatedServerProperties {
    pub fn from_file(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        match File::open(&path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                serde_java_properties::from_reader(reader).expect("Failed to parse properties")
            }
            Err(e) => {
                warn!(?e, ?path, "Failed to open properties from file");
                Self::default()
            }
        }
    }

    pub fn store(&self, path: &PathBuf) {
        match File::create(path) {
            Ok(file) => {
                let mut writer = BufWriter::new(file);
                writeln!(&mut writer, "# Minecraft server properties")
                    .expect("Failed to write comment header");
                writeln!(&mut writer, "# {}", Zoned::now())
                    .expect("Failed to write comment header");
                serde_java_properties::to_writer_with_encoding(self, writer, UTF_8)
                    .expect("Failed to serialize properties");
            }
            Err(e) => {
                error!(?e, ?path, "Failed to write properties to file");
            }
        }
    }
}
impl Default for DedicatedServerProperties {
    fn default() -> Self {
        Self {
            server_ip: "".to_string(),
            server_port: 25565,
        }
    }
}
