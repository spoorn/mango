use serde::{Deserialize, Serialize};

pub const MAIN_SERIES: &str = "main";

#[derive(Debug, Serialize, Deserialize)]
pub struct DataVersion {
    #[serde(rename = "world_version")]
    pub version: i32,
    #[serde(rename = "series_id", default = "main_series")]
    pub series: String,
}
impl DataVersion {
    pub fn new_main_series(version: i32) -> Self {
        Self {
            version,
            series: MAIN_SERIES.to_string(),
        }
    }

    pub fn new(version: i32, series: String) -> Self {
        Self { version, series }
    }
}

pub fn main_series() -> String {
    MAIN_SERIES.to_string()
}
