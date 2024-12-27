use crate::core::registries::{built_in_registries, registries};
use crate::core::{registry, Indexed};
use crate::resources::resource_key::ResourceKey;
use crate::resources::resource_location::ResourceLocation;
use crate::world::entity::animal::ocelot::Ocelot;
use crate::world::entity::animal::parrot::Parrot;
use crate::world::entity::entity::EntityTrait;
use crate::world::entity::entity_attachment::EntityAttachment;
use crate::world::entity::entity_dimensions::EntityDimensions;
use crate::world::entity::entity_type::entity_type_builder::State;
use crate::world::entity::mob_category::MobCategory;
use crate::world::level::level::Level;
use crate::world::phys::vec3::Vec3;
use bon::Builder;
use std::sync::{Arc, OnceLock};

pub static OCELOT: OnceLock<Indexed<Arc<EntityType>>> = OnceLock::new();
pub static PARROT: OnceLock<Indexed<Arc<EntityType>>> = OnceLock::new();

pub fn bootstrap() {
    OCELOT.get_or_init(|| {
        register(
            "ocelot",
            EntityType::builder()
                .sized(0.6, 0.7)
                .factory(Ocelot::boxed)
                .mob_category(MobCategory::Creature)
                .passenger_attachments(&[0.6375])
                .client_tracking_range(10)
                .build(),
        )
    });
    PARROT.get_or_init(|| {
        register(
            "parrot",
            EntityType::builder()
                .sized(0.5, 0.9)
                .factory(Parrot::boxed)
                .mob_category(MobCategory::Creature)
                .with_eye_height(0.54)
                .passenger_attachments(&[0.4625])
                .client_tracking_range(8)
                .build(),
        )
    });
}

pub type EntityFactory = fn(EntityType, Level) -> Box<dyn EntityTrait>;

#[derive(Debug, Builder)]
pub struct EntityType {
    #[builder(field = EntityDimensions::scalable(0.6, 1.8))]
    dimensions: EntityDimensions,
    factory: EntityFactory,
    mob_category: MobCategory,
    client_tracking_range: i32,
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

    pub fn with_eye_height(mut self, height: f32) -> Self {
        self.dimensions.eye_height = height;
        self
    }
}

pub fn vanilla_entity_id(path: &str) -> ResourceKey {
    ResourceKey::create(
        &registries::ENTITY_TYPE,
        ResourceLocation::with_default_namespace(path),
    )
}

pub fn register(path: &str, entity_type: EntityType) -> Indexed<Arc<EntityType>> {
    registry::register_key(
        built_in_registries::entity_type_registry(),
        vanilla_entity_id(path),
        Arc::new(entity_type),
    )
}
