use crate::detected_version::{detect_version, WorldVersion};
use std::sync::LazyLock;

pub static WORLD_VERSION: LazyLock<WorldVersion> = LazyLock::new(detect_version);

pub fn get_current_data_version() -> i32 {
    WORLD_VERSION.world_version.version
}
