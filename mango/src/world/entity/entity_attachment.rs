use crate::world::phys::vec3;
use crate::world::phys::vec3::Vec3;
use serde::Serialize;
use strum::EnumIter;

#[derive(Debug, EnumIter, Hash, Eq, PartialEq, Serialize)]
pub enum EntityAttachment {
    Passenger,
    Vehicle,
    NameTag,
    WardenChest,
}
impl EntityAttachment {
    pub fn create_fallback_points(&self, _width: f64, height: f64) -> Vec<Vec3> {
        match self {
            // AT_HEIGHT
            EntityAttachment::Passenger | EntityAttachment::NameTag => {
                vec![Vec3::new(0.0, height, 0.0)]
            }
            // AT_FEET
            EntityAttachment::Vehicle => vec![Vec3::new(0.0, height / 2.0, 0.0)],
            // AT_CENTER
            EntityAttachment::WardenChest => vec![vec3::ZERO],
        }
    }
}
