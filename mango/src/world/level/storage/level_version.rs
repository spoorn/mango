use crate::nbt::compound_tag::CompoundTag;
use crate::nbt::tag_type::TagType;
use crate::shared_constants;
use crate::util::datafix::serialization::dynamic::Dynamic;
use crate::world::level::storage::data_version;
use crate::world::level::storage::data_version::DataVersion;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LevelVersion {
    pub level_data_version: i32,
    last_played: i64,
    minecraft_version_name: String,
    minecraft_version: DataVersion,
    snapshot: bool,
}
impl LevelVersion {
    pub fn parse(dynamic: &Dynamic<CompoundTag>) -> LevelVersion {
        let version = dynamic.value.get_int("version");
        let last_played = dynamic.value.get_long("LastPlayed");
        if dynamic.value.contains("Version", TagType::CompoundTag) {
            LevelVersion {
                level_data_version: version,
                last_played,
                minecraft_version_name: dynamic
                    .value
                    .get_string_or_default("Name", shared_constants::WORLD_VERSION.name.clone()),
                minecraft_version: DataVersion {
                    version: dynamic
                        .value
                        .get_int_or_default("Id", shared_constants::get_current_data_version()),
                    series: dynamic
                        .value
                        .get_string_or_default("Series", data_version::main_series()),
                },
                snapshot: dynamic
                    .value
                    .get_bool_or_default("Snapshot", !shared_constants::WORLD_VERSION.stable),
            }
        } else {
            LevelVersion {
                level_data_version: version,
                last_played,
                minecraft_version_name: "".to_string(),
                minecraft_version: DataVersion::new(0, data_version::main_series()),
                snapshot: false,
            }
        }
    }
}
