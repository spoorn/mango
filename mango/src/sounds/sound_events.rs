//! These are LazyLock for ergonomics and I don't yet see a reason to have them behind an Arc

use crate::core::registries::built_in_registries;
use crate::core::registry;
use crate::resources::resource_location::ResourceLocation;
use crate::sounds::sound_event::SoundEvent;
use std::sync::LazyLock;

pub static WOOL_BREAK: LazyLock<SoundEvent> = LazyLock::new(|| register_path("block.wool.break"));
pub static WOOL_STEP: LazyLock<SoundEvent> = LazyLock::new(|| register_path("block.wool.step"));
pub static WOOL_PLACE: LazyLock<SoundEvent> = LazyLock::new(|| register_path("block.wool.place"));
pub static WOOL_HIT: LazyLock<SoundEvent> = LazyLock::new(|| register_path("block.wool.hit"));
pub static WOOL_FALL: LazyLock<SoundEvent> = LazyLock::new(|| register_path("block.wool.fall"));
pub static COBWEB_BREAK: LazyLock<SoundEvent> =
    LazyLock::new(|| register_path("block.cobweb.break"));
pub static COBWEB_STEP: LazyLock<SoundEvent> = LazyLock::new(|| register_path("block.cobweb.step"));
pub static COBWEB_PLACE: LazyLock<SoundEvent> =
    LazyLock::new(|| register_path("block.cobweb.place"));
pub static COBWEB_HIT: LazyLock<SoundEvent> = LazyLock::new(|| register_path("block.cobweb.hit"));
pub static COBWEB_FALL: LazyLock<SoundEvent> = LazyLock::new(|| register_path("block.cobweb.fall"));
pub static BAMBOO_SAPLING_BREAK: LazyLock<SoundEvent> =
    LazyLock::new(|| register_path("block.bamboo_sapling.break"));
pub static BAMBOO_STEP: LazyLock<SoundEvent> = LazyLock::new(|| register_path("block.bamboo.step"));
pub static BAMBOO_SAPLING_PLACE: LazyLock<SoundEvent> =
    LazyLock::new(|| register_path("block.bamboo_sapling.place"));
pub static BAMBOO_SAPLING_HIT: LazyLock<SoundEvent> =
    LazyLock::new(|| register_path("block.bamboo_sapling.hit"));
pub static BAMBOO_FALL: LazyLock<SoundEvent> = LazyLock::new(|| register_path("block.bamboo.fall"));
pub static GRASS_BREAK: LazyLock<SoundEvent> = LazyLock::new(|| register_path("block.grass.break"));
pub static GRASS_STEP: LazyLock<SoundEvent> = LazyLock::new(|| register_path("block.grass.step"));
pub static GRASS_PLACE: LazyLock<SoundEvent> = LazyLock::new(|| register_path("block.grass.place"));
pub static GRASS_HIT: LazyLock<SoundEvent> = LazyLock::new(|| register_path("block.grass.hit"));
pub static GRASS_FALL: LazyLock<SoundEvent> = LazyLock::new(|| register_path("block.grass.fall"));

fn register_path(path: &str) -> SoundEvent {
    register_location(ResourceLocation::with_default_namespace(path))
}

fn register_location(location: ResourceLocation) -> SoundEvent {
    let sound_event = SoundEvent::create_variable_range_event(location.clone());
    registry::register_location_take(
        built_in_registries::sound_event_registry(),
        location,
        sound_event.clone(),
    );
    sound_event
}

// TODO: Not needed
// fn register(location: ResourceLocation, variable_range_location: ResourceLocation) {
//
// }
