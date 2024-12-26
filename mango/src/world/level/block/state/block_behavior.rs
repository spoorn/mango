use crate::resources::resource_key::ResourceKey;
use crate::world::level::block::sound_type::SoundType;
use crate::world::level::block::state::block_behavior::properties_builder::{
    IsUnset, SetCanOcclude, SetDestroyTime, SetExplosionResistance, SetHasCollision, State,
};
use crate::world::level::material::map_color;
use crate::world::level::material::map_color::MapColor;
use crate::world::level::material::push_reaction::PushReaction;
use bon::Builder;

#[derive(Builder)]
#[builder(state_mod(vis = "pub"))]
pub struct Properties {
    pub id: Option<ResourceKey>,
    #[builder(default = |_| map_color::NONE)]
    pub map_color: fn(BlockState) -> MapColor,
    #[builder(default = true)]
    pub has_collision: bool,
    pub sound_type: SoundType,
    /// `light_emission` in vanilla
    #[builder(default = |_| 0)]
    pub light_level: fn(BlockState) -> u8,
    #[builder(default = true)]
    pub can_occlude: bool,
    pub destroy_time: f32,
    #[builder(with = |r: f32| f32::max(0.0, r))]
    pub explosion_resistance: f32,
    #[builder(default)]
    pub push_reaction: PushReaction,
    pub replaceable: bool,
}

impl<S: State> PropertiesBuilder<S> {
    pub fn no_collision(self) -> PropertiesBuilder<SetCanOcclude<SetHasCollision<S>>>
    where
        S::HasCollision: IsUnset,
        S::CanOcclude: IsUnset,
    {
        self.has_collision(false).can_occlude(false)
    }

    pub fn instabreak(self) -> PropertiesBuilder<SetExplosionResistance<SetDestroyTime<S>>>
    where
        S::ExplosionResistance: IsUnset,
        S::DestroyTime: IsUnset,
    {
        self.strength(0.0)
    }

    pub fn strength(
        self,
        strength: f32,
    ) -> PropertiesBuilder<SetExplosionResistance<SetDestroyTime<S>>>
    where
        S::ExplosionResistance: IsUnset,
        S::DestroyTime: IsUnset,
    {
        self.strength_with_res(strength, strength)
    }

    pub fn strength_with_res(
        self,
        destroy_time: f32,
        explosion_resistance: f32,
    ) -> PropertiesBuilder<SetExplosionResistance<SetDestroyTime<S>>>
    where
        S::ExplosionResistance: IsUnset,
        S::DestroyTime: IsUnset,
    {
        self.destroy_time(destroy_time)
            .explosion_resistance(explosion_resistance)
    }
}

pub struct BlockState {}
