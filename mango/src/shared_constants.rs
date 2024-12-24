use std::sync::LazyLock;
use crate::detected_version::{detect_version, WorldVersion};

pub static WORLD_VERSION: LazyLock<WorldVersion> = LazyLock::new(detect_version);
