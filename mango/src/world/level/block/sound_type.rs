//! These are LazyLock for ergonomics and I don't yet see a reason to have them behind an Arc

use crate::sounds::sound_event::SoundEvent;
use crate::sounds::sound_events;
use serde::Serialize;
use std::clone::Clone;
use std::sync::LazyLock;

pub static WOOL: LazyLock<SoundType> = LazyLock::new(|| SoundType {
    volume: 1.0,
    pitch: 1.0,
    break_sound: sound_events::WOOL_BREAK.clone(),
    step_sound: sound_events::WOOL_STEP.clone(),
    place_sound: sound_events::WOOL_PLACE.clone(),
    hit_sound: sound_events::WOOL_HIT.clone(),
    fall_sound: sound_events::WOOL_FALL.clone(),
});
pub static COBWEB: LazyLock<SoundType> = LazyLock::new(|| SoundType {
    volume: 1.0,
    pitch: 1.0,
    break_sound: sound_events::COBWEB_BREAK.clone(),
    step_sound: sound_events::COBWEB_STEP.clone(),
    place_sound: sound_events::COBWEB_PLACE.clone(),
    hit_sound: sound_events::COBWEB_HIT.clone(),
    fall_sound: sound_events::COBWEB_FALL.clone(),
});
pub static BAMBOO_SAPLING: LazyLock<SoundType> = LazyLock::new(|| SoundType {
    volume: 1.0,
    pitch: 1.0,
    break_sound: sound_events::BAMBOO_SAPLING_BREAK.clone(),
    step_sound: sound_events::BAMBOO_STEP.clone(),
    place_sound: sound_events::BAMBOO_SAPLING_PLACE.clone(),
    hit_sound: sound_events::BAMBOO_SAPLING_HIT.clone(),
    fall_sound: sound_events::BAMBOO_FALL.clone(),
});
pub static GRASS: LazyLock<SoundType> = LazyLock::new(|| SoundType {
    volume: 1.0,
    pitch: 1.0,
    break_sound: sound_events::GRASS_BREAK.clone(),
    step_sound: sound_events::GRASS_STEP.clone(),
    place_sound: sound_events::GRASS_PLACE.clone(),
    hit_sound: sound_events::GRASS_HIT.clone(),
    fall_sound: sound_events::GRASS_FALL.clone(),
});

#[derive(Clone, Debug, Serialize)]
pub struct SoundType {
    pub volume: f32,
    pub pitch: f32,
    pub break_sound: SoundEvent,
    pub step_sound: SoundEvent,
    pub place_sound: SoundEvent,
    pub hit_sound: SoundEvent,
    pub fall_sound: SoundEvent,
}
