use crate::world::entity::entity::EntityTrait;
use crate::world::entity::entity_type::EntityType;
use crate::world::level::level::Level;

pub struct Ocelot {}
impl EntityTrait for Ocelot {}
impl Ocelot {
    fn new(entity_type: EntityType, level: Level) -> Self {
        Self {}
    }

    pub fn boxed_option(entity_type: EntityType, level: Level) -> Option<Box<dyn EntityTrait>> {
        Some(Box::new(Self::new(entity_type, level)))
    }
}
