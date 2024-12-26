use crate::resources::resource_key::ResourceKey;
use crate::world::level::block::sound_type::SoundType;
use crate::world::level::block::state::block_behavior::properties_builder::{
    SetCanOcclude, SetDestroyTime, SetExplosionResistance, SetHasCollision, State,
};
use bon::Builder;
use bon::__::IsUnset;

#[derive(Builder)]
#[builder(state_mod(vis = "pub"))]
pub struct Properties {
    pub id: Option<ResourceKey>,
    pub replaceable: bool,
    #[builder(default = true)]
    pub has_collision: bool,
    pub sound_type: SoundType,
    #[builder(default = true)]
    pub can_occlude: bool,
    pub destroy_time: f32,
    #[builder(with = |r: f32| f32::max(0.0, r))]
    pub explosion_resistance: f32,
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
