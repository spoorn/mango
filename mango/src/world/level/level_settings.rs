use crate::nbt::compound_tag::CompoundTag;
use crate::util::datafix::serialization::dynamic::Dynamic;
use crate::world::difficulty::Difficulty;
use crate::world::level::game_rules::GameRules;
use crate::world::level::game_type::GameType;
use crate::world::level::world_data_configuration::WorldDataConfiguration;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LevelSettings {
    level_name: String,
    game_type: GameType,
    hardcore: bool,
    difficulty: Difficulty,
    allow_commands: bool,
    game_rules: GameRules,
    pub data_configuration: WorldDataConfiguration,
}
impl LevelSettings {
    pub fn new(
        level_name: String,
        game_type: GameType,
        hardcore: bool,
        difficulty: Difficulty,
        allow_commands: bool,
        game_rules: GameRules,
        data_configuration: WorldDataConfiguration,
    ) -> Self {
        Self {
            level_name,
            game_type,
            hardcore,
            difficulty,
            allow_commands,
            game_rules,
            data_configuration,
        }
    }

    pub fn parse(
        dynamic: &Dynamic<CompoundTag>,
        data_configuration: WorldDataConfiguration,
    ) -> Self {
        let game_type = GameType::by_id(
            dynamic
                .value
                .get_int_or_default("GameType", GameType::Survival as i32),
        );
        Self {
            level_name: dynamic
                .value
                .get_string_or_default("LevelName", "".to_string()),
            hardcore: dynamic.value.get_bool_or_default("hardcore", false),
            difficulty: Difficulty::by_id(
                dynamic
                    .value
                    .get_int_or_default("Difficulty", Difficulty::Normal as i32),
            ),
            allow_commands: dynamic
                .value
                .get_bool_or_default("allowCommands", game_type == GameType::Creative),
            game_type,
            game_rules: GameRules::new(
                data_configuration.enabled_features.clone(),
                dynamic.value.get_compound("GameRules"),
            ),
            data_configuration,
        }
    }
}
