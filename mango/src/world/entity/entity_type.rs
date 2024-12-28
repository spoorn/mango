use crate::core::registries::{built_in_registries, registries};
use crate::core::{registry, GlobalIndexed, Indexed};
use crate::resources::resource_key::ResourceKey;
use crate::resources::resource_location::ResourceLocation;
use crate::world::entity::animal::ocelot::Ocelot;
use crate::world::entity::animal::parrot::Parrot;
use crate::world::entity::entity::EntityTrait;
use crate::world::entity::entity_attachment::EntityAttachment;
use crate::world::entity::entity_dimensions::EntityDimensions;
use crate::world::entity::entity_type::entity_type_builder::{
    IsUnset, SetSerialize, SetSummon, State,
};
use crate::world::entity::mob_category::MobCategory;
use crate::world::entity::player::player;
use crate::world::level::level::Level;
use crate::world::phys::vec3::Vec3;
use bon::Builder;
use serde::Serialize;
use std::sync::Arc;

pub static OCELOT: GlobalIndexed<EntityType> = GlobalIndexed::new(|| {
    register(
        "ocelot",
        EntityType::builder()
            .sized(0.6, 0.7)
            .factory(Ocelot::boxed_option)
            .mob_category(MobCategory::Creature)
            .passenger_attachments(&[0.6375])
            .client_tracking_range(10)
            .build(),
    )
});
pub static PARROT: GlobalIndexed<EntityType> = GlobalIndexed::new(|| {
    register(
        "parrot",
        EntityType::builder()
            .sized(0.5, 0.9)
            .factory(Parrot::boxed_option)
            .mob_category(MobCategory::Creature)
            .with_eye_height(0.54)
            .passenger_attachments(&[0.4625])
            .client_tracking_range(8)
            .build(),
    )
});
pub static PLAYER: GlobalIndexed<EntityType> = GlobalIndexed::new(|| {
    register(
        "player",
        EntityType::builder()
            .factory(|_, _| None)
            .mob_category(MobCategory::Misc)
            .no_save()
            .no_summon()
            .with_eye_height(1.62)
            .vehicle_attachment(player::DEFAULT_VEHICLE_ATTACHMENT)
            .client_tracking_range(32)
            .update_interval(2)
            .build(),
    )
});

pub fn bootstrap() {
    OCELOT.init();
    PARROT.init();
    PLAYER.init();
}

pub type EntityFactory = fn(EntityType, Level) -> Option<Box<dyn EntityTrait>>;

#[derive(Debug, Builder, Serialize)]
pub struct EntityType {
    #[builder(field = EntityDimensions::scalable(0.6, 1.8))]
    dimensions: EntityDimensions,
    #[serde(skip)]
    factory: EntityFactory,
    mob_category: MobCategory,
    #[builder(default = true)]
    serialize: bool,
    #[builder(default = true)]
    summon: bool,
    client_tracking_range: i32,
    #[builder(default = 3)]
    update_interval: i32,
}
impl<S: State> EntityTypeBuilder<S> {
    pub fn sized(mut self, width: f32, height: f32) -> Self {
        self.dimensions = EntityDimensions::scalable(width, height);
        self
    }

    pub fn passenger_attachments(mut self, heights: &[f32]) -> Self {
        heights.iter().for_each(|height| {
            self.dimensions.attach(
                EntityAttachment::Passenger,
                Vec3::new(0.0, *height as f64, 0.0),
            );
        });
        self
    }

    pub fn vehicle_attachment(mut self, rel_pos: Vec3) -> Self {
        self.dimensions.attach(EntityAttachment::Vehicle, rel_pos);
        self
    }

    pub fn with_eye_height(mut self, height: f32) -> Self {
        self.dimensions.eye_height = height;
        self
    }

    pub fn no_save(self) -> EntityTypeBuilder<SetSerialize<S>>
    where
        S::Serialize: IsUnset,
    {
        self.serialize(false)
    }

    pub fn no_summon(self) -> EntityTypeBuilder<SetSummon<S>>
    where
        S::Summon: IsUnset,
    {
        self.summon(false)
    }
}

pub fn vanilla_entity_id(path: &str) -> ResourceKey {
    ResourceKey::create(
        &registries::ENTITY_TYPE,
        ResourceLocation::with_default_namespace(path),
    )
}

pub fn register(path: &str, entity_type: EntityType) -> Indexed<EntityType> {
    registry::register_key(
        built_in_registries::entity_type_registry(),
        vanilla_entity_id(path),
        Arc::new(entity_type),
    )
}
