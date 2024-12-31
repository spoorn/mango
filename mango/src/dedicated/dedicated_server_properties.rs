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
#[serde(rename_all = "kebab-case")]
pub struct DedicatedServerProperties {
    pub server_ip: String,
    pub server_port: u16,
    /// Root of the Minecraft world instance
    #[serde(default = "default_universe")]
    pub universe: String,
    /// Minecraft world name, and name of the world folder
    #[serde(default = "default_level_name")]
    pub level_name: String,
    /// true to use vanilla datapacks only, else false
    #[serde(default)]
    pub safe_mode: bool,
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
            universe: ".".to_string(),
            level_name: "world".to_string(),
            safe_mode: false,
        }
    }
}

/*
    Serde does not support default literals or expressions: https://github.com/serde-rs/serde/issues/368.
    This is a workaround to specify a tiny function for each. We need these default values as these
    are not required as part of the server.properties file and can instead be command args, but we
    don't want to break deserialization or skip it.

    We also don't yet support command line args.
*/

fn default_universe() -> String {
    ".".to_string()
}

fn default_level_name() -> String {
    "world".to_string()
}
