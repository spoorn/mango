use crate::core::Global;
use crate::minecraft_server::MinecraftServer;
use crate::nbt::compound_tag::CompoundTag;
use crate::world::flag::feature_flag_set::FeatureFlagSet;
use crate::world::flag::feature_flags;
use dashmap::DashMap;
use num::cast::AsPrimitive;
use serde::{Serialize, Serializer};
use serde_with::SerializeDisplay;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use strum::Display;

// TODO: does this need to be a DashMap?
static GAME_RULE_TYPES: Global<DashMap<GameRuleKey, GameRuleValue>> =
    Global::new(|| DashMap::new());
pub static RULE_DOFIRETICK: Global<GameRuleKey> = Global::new(|| {
    register(
        "doFireTick",
        GameRuleCategory::Updates,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_MOBGRIEFING: Global<GameRuleKey> = Global::new(|| {
    register(
        "mobGriefing",
        GameRuleCategory::Mobs,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_KEEPINVENTORY: Global<GameRuleKey> = Global::new(|| {
    register(
        "keepInventory",
        GameRuleCategory::Player,
        GameRuleValue::create(GameRuleValueTypes::Boolean(false)),
    )
});
pub static RULE_DOMOBSPAWNING: Global<GameRuleKey> = Global::new(|| {
    register(
        "doMobSpawning",
        GameRuleCategory::Spawning,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_DOMOBLOOT: Global<GameRuleKey> = Global::new(|| {
    register(
        "doMobLoot",
        GameRuleCategory::Drops,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_PROJECTILESCANBREAKBLOCKS: Global<GameRuleKey> = Global::new(|| {
    register(
        "projectilesCanBreakBlocks",
        GameRuleCategory::Drops,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_DOBLOCKDROPS: Global<GameRuleKey> = Global::new(|| {
    register(
        "doTileDrops",
        GameRuleCategory::Drops,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_DOENTITYDROPS: Global<GameRuleKey> = Global::new(|| {
    register(
        "doEntityDrops",
        GameRuleCategory::Drops,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_COMMANDBLOCKOUTPUT: Global<GameRuleKey> = Global::new(|| {
    register(
        "commandBlockOutput",
        GameRuleCategory::Chat,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_NATURAL_REGENERATION: Global<GameRuleKey> = Global::new(|| {
    register(
        "naturalRegeneration",
        GameRuleCategory::Player,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_DAYLIGHT: Global<GameRuleKey> = Global::new(|| {
    register(
        "doDaylightCycle",
        GameRuleCategory::Updates,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_LOGADMINCOMMANDS: Global<GameRuleKey> = Global::new(|| {
    register(
        "logAdminCommands",
        GameRuleCategory::Chat,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_SHOWDEATHMESSAGES: Global<GameRuleKey> = Global::new(|| {
    register(
        "showDeathMessages",
        GameRuleCategory::Chat,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_RANDOMTICKING: Global<GameRuleKey> = Global::new(|| {
    register(
        "randomTickSpeed",
        GameRuleCategory::Updates,
        GameRuleValue::create(GameRuleValueTypes::Integer(3)),
    )
});
pub static RULE_SENDCOMMANDFEEDBACK: Global<GameRuleKey> = Global::new(|| {
    register(
        "sendCommandFeedback",
        GameRuleCategory::Chat,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_REDUCEDDEBUGINFO: Global<GameRuleKey> = Global::new(|| {
    register(
        "reducedDebugInfo",
        GameRuleCategory::Misc,
        GameRuleValue::create_with_callback(GameRuleValueTypes::Boolean(false), |server, value| {
            todo!("Handle debug packet");
        }),
    )
});
pub static RULE_SPECTATORSGENERATECHUNKS: Global<GameRuleKey> = Global::new(|| {
    register(
        "spectatorsGenerateChunks",
        GameRuleCategory::Player,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_SPAWN_RADIUS: Global<GameRuleKey> = Global::new(|| {
    register(
        "spawnRadius",
        GameRuleCategory::Player,
        GameRuleValue::create(GameRuleValueTypes::Integer(10)),
    )
});
pub static RULE_DISABLE_PLAYER_MOVEMENT_CHECK: Global<GameRuleKey> = Global::new(|| {
    register(
        "disablePlayerMovementCheck",
        GameRuleCategory::Player,
        GameRuleValue::create(GameRuleValueTypes::Boolean(false)),
    )
});
pub static RULE_DISABLE_ELYTRA_MOVEMENT_CHECK: Global<GameRuleKey> = Global::new(|| {
    register(
        "disableElytraMovementCheck",
        GameRuleCategory::Player,
        GameRuleValue::create(GameRuleValueTypes::Boolean(false)),
    )
});
pub static RULE_MAX_ENTITY_CRAMMING: Global<GameRuleKey> = Global::new(|| {
    register(
        "maxEntityCramming",
        GameRuleCategory::Mobs,
        GameRuleValue::create(GameRuleValueTypes::Integer(24)),
    )
});
pub static RULE_WEATHER_CYCLE: Global<GameRuleKey> = Global::new(|| {
    register(
        "doWeatherCycle",
        GameRuleCategory::Updates,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_LIMITED_CRAFTING: Global<GameRuleKey> = Global::new(|| {
    register(
        "doLimitedCrafting",
        GameRuleCategory::Player,
        GameRuleValue::create_with_callback(GameRuleValueTypes::Boolean(false), |server, value| {
            todo!("Handle limited crafting rule");
        }),
    )
});
pub static RULE_MAX_COMMAND_CHAIN_LENGTH: Global<GameRuleKey> = Global::new(|| {
    register(
        "maxCommandChainLength",
        GameRuleCategory::Misc,
        GameRuleValue::create(GameRuleValueTypes::Integer(65536)),
    )
});
pub static RULE_MAX_COMMAND_FORK_COUNT: Global<GameRuleKey> = Global::new(|| {
    register(
        "maxCommandForkCount",
        GameRuleCategory::Misc,
        GameRuleValue::create(GameRuleValueTypes::Integer(65536)),
    )
});
pub static RULE_COMMAND_MODIFICATION_BLOCK_LIMIT: Global<GameRuleKey> = Global::new(|| {
    register(
        "commandModificationBlockLimit",
        GameRuleCategory::Misc,
        GameRuleValue::create(GameRuleValueTypes::Integer(32768)),
    )
});
pub static RULE_ANNOUNCE_ADVANCEMENTS: Global<GameRuleKey> = Global::new(|| {
    register(
        "announceAdvancements",
        GameRuleCategory::Chat,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_DISABLE_RAIDS: Global<GameRuleKey> = Global::new(|| {
    register(
        "disableRaids",
        GameRuleCategory::Mobs,
        GameRuleValue::create(GameRuleValueTypes::Boolean(false)),
    )
});
pub static RULE_DOINSOMNIA: Global<GameRuleKey> = Global::new(|| {
    register(
        "doInsomnia",
        GameRuleCategory::Spawning,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_DO_IMMEDIATE_RESPAWN: Global<GameRuleKey> = Global::new(|| {
    register(
        "doImmediateRespawn",
        GameRuleCategory::Player,
        GameRuleValue::create_with_callback(GameRuleValueTypes::Boolean(false), |server, value| {
            todo!("Handle immediate respawn rule");
        }),
    )
});
pub static RULE_PLAYERS_NETHER_PORTAL_DEFAULT_DELAY: Global<GameRuleKey> = Global::new(|| {
    register(
        "playersNetherPortalDefaultDelay",
        GameRuleCategory::Player,
        GameRuleValue::create(GameRuleValueTypes::Integer(80)),
    )
});
pub static RULE_PLAYERS_NETHER_PORTAL_CREATIVE_DELAY: Global<GameRuleKey> = Global::new(|| {
    register(
        "playersNetherPortalCreativeDelay",
        GameRuleCategory::Player,
        GameRuleValue::create(GameRuleValueTypes::Integer(0)),
    )
});
pub static RULE_DROWNING_DAMAGE: Global<GameRuleKey> = Global::new(|| {
    register(
        "drowningDamage",
        GameRuleCategory::Player,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_FALL_DAMAGE: Global<GameRuleKey> = Global::new(|| {
    register(
        "fallDamage",
        GameRuleCategory::Player,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_FIRE_DAMAGE: Global<GameRuleKey> = Global::new(|| {
    register(
        "fireDamage",
        GameRuleCategory::Player,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_FREEZE_DAMAGE: Global<GameRuleKey> = Global::new(|| {
    register(
        "freezeDamage",
        GameRuleCategory::Player,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_DO_PATROL_SPAWNING: Global<GameRuleKey> = Global::new(|| {
    register(
        "doPatrolSpawning",
        GameRuleCategory::Spawning,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_DO_TRADER_SPAWNING: Global<GameRuleKey> = Global::new(|| {
    register(
        "doTraderSpawning",
        GameRuleCategory::Spawning,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_DO_WARDEN_SPAWNING: Global<GameRuleKey> = Global::new(|| {
    register(
        "doWardenSpawning",
        GameRuleCategory::Spawning,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_FORGIVE_DEAD_PLAYERS: Global<GameRuleKey> = Global::new(|| {
    register(
        "forgiveDeadPlayers",
        GameRuleCategory::Mobs,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_UNIVERSAL_ANGER: Global<GameRuleKey> = Global::new(|| {
    register(
        "universalAnger",
        GameRuleCategory::Mobs,
        GameRuleValue::create(GameRuleValueTypes::Boolean(false)),
    )
});
pub static RULE_PLAYERS_SLEEPING_PERCENTAGE: Global<GameRuleKey> = Global::new(|| {
    register(
        "playersSleepingPercentage",
        GameRuleCategory::Player,
        GameRuleValue::create(GameRuleValueTypes::Integer(100)),
    )
});
pub static RULE_BLOCK_EXPLOSION_DROP_DECAY: Global<GameRuleKey> = Global::new(|| {
    register(
        "blockExplosionDropDecay",
        GameRuleCategory::Drops,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_MOB_EXPLOSION_DROP_DECAY: Global<GameRuleKey> = Global::new(|| {
    register(
        "mobExplosionDropDecay",
        GameRuleCategory::Drops,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_TNT_EXPLOSION_DROP_DECAY: Global<GameRuleKey> = Global::new(|| {
    register(
        "tntExplosionDropDecay",
        GameRuleCategory::Drops,
        GameRuleValue::create(GameRuleValueTypes::Boolean(false)),
    )
});
pub static RULE_SNOW_ACCUMULATION_HEIGHT: Global<GameRuleKey> = Global::new(|| {
    register(
        "snowAccumulationHeight",
        GameRuleCategory::Updates,
        GameRuleValue::create(GameRuleValueTypes::Integer(1)),
    )
});
pub static RULE_WATER_SOURCE_CONVERSION: Global<GameRuleKey> = Global::new(|| {
    register(
        "waterSourceConversion",
        GameRuleCategory::Updates,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_LAVA_SOURCE_CONVERSION: Global<GameRuleKey> = Global::new(|| {
    register(
        "lavaSourceConversion",
        GameRuleCategory::Updates,
        GameRuleValue::create(GameRuleValueTypes::Boolean(false)),
    )
});
pub static RULE_GLOBAL_SOUND_EVENTS: Global<GameRuleKey> = Global::new(|| {
    register(
        "globalSoundEvents",
        GameRuleCategory::Misc,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
pub static RULE_DO_VINES_SPREAD: Global<GameRuleKey> = Global::new(|| {
    register(
        "doVinesSpread",
        GameRuleCategory::Updates,
        GameRuleValue::create(GameRuleValueTypes::Boolean(true)),
    )
});
// TODO: commands
pub static RULE_MINECART_MAX_SPEED: Global<GameRuleKey> = Global::new(|| {
    register(
        "minecartMaxSpeed",
        GameRuleCategory::Misc,
        GameRuleValue::create_with_feature_flags(
            GameRuleValueTypes::Integer(8),
            FeatureFlagSet::from(&feature_flags::FEATURE_FLAGS.minecart_improvements),
        ),
    )
});
// TODO: commands
pub static RULE_SPAWN_CHUNK_RADIUS: Global<GameRuleKey> = Global::new(|| {
    register(
        "spawnChunkRadius",
        GameRuleCategory::Misc,
        GameRuleValue::create_with_callback(GameRuleValueTypes::Integer(2), |server, value| {
            todo!("Handle spawn chunk radius rule");
        }),
    )
});

pub fn bootstrap() {
    RULE_DOFIRETICK.init();
    RULE_MOBGRIEFING.init();
    RULE_KEEPINVENTORY.init();
    RULE_DOMOBSPAWNING.init();
    RULE_DOMOBLOOT.init();
    RULE_PROJECTILESCANBREAKBLOCKS.init();
    RULE_DOBLOCKDROPS.init();
    RULE_DOENTITYDROPS.init();
    RULE_COMMANDBLOCKOUTPUT.init();
    RULE_NATURAL_REGENERATION.init();
    RULE_DAYLIGHT.init();
    RULE_LOGADMINCOMMANDS.init();
    RULE_SHOWDEATHMESSAGES.init();
    RULE_RANDOMTICKING.init();
    RULE_SENDCOMMANDFEEDBACK.init();
    RULE_REDUCEDDEBUGINFO.init();
    RULE_SPECTATORSGENERATECHUNKS.init();
    RULE_SPAWN_RADIUS.init();
    RULE_DISABLE_PLAYER_MOVEMENT_CHECK.init();
    RULE_DISABLE_ELYTRA_MOVEMENT_CHECK.init();
    RULE_MAX_ENTITY_CRAMMING.init();
    RULE_WEATHER_CYCLE.init();
    RULE_LIMITED_CRAFTING.init();
    RULE_MAX_COMMAND_CHAIN_LENGTH.init();
    RULE_MAX_COMMAND_FORK_COUNT.init();
    RULE_COMMAND_MODIFICATION_BLOCK_LIMIT.init();
    RULE_ANNOUNCE_ADVANCEMENTS.init();
    RULE_DISABLE_RAIDS.init();
    RULE_DOINSOMNIA.init();
    RULE_DO_IMMEDIATE_RESPAWN.init();
    RULE_PLAYERS_NETHER_PORTAL_DEFAULT_DELAY.init();
    RULE_PLAYERS_NETHER_PORTAL_CREATIVE_DELAY.init();
    RULE_DROWNING_DAMAGE.init();
    RULE_FALL_DAMAGE.init();
    RULE_FIRE_DAMAGE.init();
    RULE_FREEZE_DAMAGE.init();
    RULE_DO_PATROL_SPAWNING.init();
    RULE_DO_TRADER_SPAWNING.init();
    RULE_DO_WARDEN_SPAWNING.init();
    RULE_FORGIVE_DEAD_PLAYERS.init();
    RULE_UNIVERSAL_ANGER.init();
    RULE_PLAYERS_SLEEPING_PERCENTAGE.init();
    RULE_BLOCK_EXPLOSION_DROP_DECAY.init();
    RULE_MOB_EXPLOSION_DROP_DECAY.init();
    RULE_TNT_EXPLOSION_DROP_DECAY.init();
    RULE_SNOW_ACCUMULATION_HEIGHT.init();
    RULE_WATER_SOURCE_CONVERSION.init();
    RULE_LAVA_SOURCE_CONVERSION.init();
    RULE_GLOBAL_SOUND_EVENTS.init();
    RULE_DO_VINES_SPREAD.init();
    RULE_MINECART_MAX_SPEED.init();
    RULE_SPAWN_CHUNK_RADIUS.init();
}

/// An instance of game rules, filtered from the global GAME_RULE_TYPES
#[derive(Debug, Serialize)]
pub struct GameRules {
    rules: HashMap<GameRuleKey, GameRuleValue>,
    enabled_features: FeatureFlagSet,
}
impl GameRules {
    pub fn new(feature_flag_set: FeatureFlagSet, compound_tag: CompoundTag) -> Self {
        let mut res = Self::from(feature_flag_set);
        res.rules.iter_mut().for_each(|(key, value)| {
            if let Some(tag) = compound_tag.try_get_string(key.id) {
                // Try parse as i32, then bool as those are the only game rule value types
                if let Ok(parsed) = tag.parse::<i32>() {
                    value.value = GameRuleValueTypes::Integer(parsed);
                } else if let Ok(parsed) = tag.parse::<bool>() {
                    value.value = GameRuleValueTypes::Boolean(parsed);
                }
            }
        });
        res
    }
}
impl From<FeatureFlagSet> for GameRules {
    fn from(value: FeatureFlagSet) -> Self {
        let available_rules = GAME_RULE_TYPES
            .iter()
            .filter(|e| e.game_rule_type.required_features.is_subset_of(&value))
            .map(|e| (*e.key(), e.clone()))
            .collect();
        Self {
            rules: available_rules,
            enabled_features: value,
        }
    }
}

/// Keys hashed and primarily ordered by their id
#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, SerializeDisplay)]
pub struct GameRuleKey {
    id: &'static str,
    category: GameRuleCategory,
}
impl GameRuleKey {
    pub const fn new(id: &'static str, category: GameRuleCategory) -> Self {
        Self { id, category }
    }
}
impl Hash for GameRuleKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl Display for GameRuleKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}|category={}", self.id, self.category)
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, Ord, PartialOrd, Display)]
pub enum GameRuleCategory {
    Player,
    Mobs,
    Spawning,
    Drops,
    Updates,
    Chat,
    Misc,
}
impl GameRuleCategory {
    pub fn get_description_id(&self) -> &'static str {
        match self {
            GameRuleCategory::Player => "gamerule.category.player",
            GameRuleCategory::Mobs => "gamerule.category.mobs",
            GameRuleCategory::Spawning => "gamerule.category.spawning",
            GameRuleCategory::Drops => "gamerule.category.drops",
            GameRuleCategory::Updates => "gamerule.category.updates",
            GameRuleCategory::Chat => "gamerule.category.chat",
            GameRuleCategory::Misc => "gamerule.category.misc",
        }
    }
}

/// Blanket trait alias to allow using in trait objects
pub trait GameRuleValueType: AsPrimitive<i32> + Debug + Send + Sync {}
impl<T: AsPrimitive<i32> + Debug + Send + Sync> GameRuleValueType for T {}

// trait GameRuleValueTrait: Send + Sync {
//     fn set_value<T>(&mut self, value: T);
//
//     fn get_value<T>(&self) -> &T;
//
//     fn get_game_rule_type<T>(&self) -> &GameRuleType<T>;
// }

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(untagged)]
pub enum GameRuleValueTypes {
    Integer(i32),
    Boolean(bool),
}

#[derive(Clone, Debug, Serialize)]
pub struct GameRuleValue {
    game_rule_type: GameRuleType,
    pub value: GameRuleValueTypes,
}
impl GameRuleValue {
    pub fn create(value: GameRuleValueTypes) -> Self {
        Self::create_with_callback(value, |_, _| ())
    }

    pub fn create_with_feature_flags(
        value: GameRuleValueTypes,
        feature_flag_set: FeatureFlagSet,
    ) -> Self {
        Self {
            game_rule_type: GameRuleType {
                callback: |_, _| (),
                required_features: feature_flag_set,
            },
            value,
        }
    }

    pub fn create_with_callback(
        value: GameRuleValueTypes,
        callback: fn(MinecraftServer, GameRuleValueTypes) -> (),
    ) -> Self {
        Self {
            game_rule_type: GameRuleType {
                callback,
                required_features: FeatureFlagSet::empty(),
            },
            value,
        }
    }
}
// impl GameRuleValueTrait for GameRuleValue<i32> {
//     fn set_value<T: GameRuleValueType>(&mut self, value: T) {
//         todo!()
//     }
//
//     fn get_value<T: GameRuleValueType>(&self) -> &T {
//         todo!()
//     }
//
//     fn get_game_rule_type<T: GameRuleValueType>(&self) -> &GameRuleType<T> {
//         todo!()
//     }
// }

/// TODO: visitorCaller
#[derive(Clone, Debug, Serialize)]
pub struct GameRuleType {
    #[serde(skip)]
    callback: fn(MinecraftServer, GameRuleValueTypes) -> (),
    required_features: FeatureFlagSet,
}

fn register(id: &'static str, category: GameRuleCategory, value: GameRuleValue) -> GameRuleKey {
    let key = GameRuleKey::new(id, category);
    if let Some(prev) = GAME_RULE_TYPES.insert(key, value) {
        panic!("Illegal game rule registration for {}: {:?}", id, prev);
    }
    key
}
