use crate::world::level::level_settings::LevelSettings;
use crate::world::level::storage::level_version::LevelVersion;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct LevelSummary {
    settings: LevelSettings,
    level_version: LevelVersion,
    level_id: String,
    requires_manual_conversion: bool,
    locked: bool,
    experimental: bool,
    icon: PathBuf,
}
impl LevelSummary {
    pub fn new(
        settings: LevelSettings,
        level_version: LevelVersion,
        level_id: String,
        requires_manual_conversion: bool,
        locked: bool,
        experimental: bool,
        icon: PathBuf,
    ) -> Self {
        Self {
            settings,
            level_version,
            level_id,
            requires_manual_conversion,
            locked,
            experimental,
            icon,
        }
    }
}
