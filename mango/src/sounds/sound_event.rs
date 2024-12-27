use crate::resources::resource_location::ResourceLocation;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SoundEvent {
    pub location: ResourceLocation,
    pub fixed_range: Option<f32>,
}

impl SoundEvent {
    pub fn create_variable_range_event(location: ResourceLocation) -> Self {
        Self {
            location,
            fixed_range: None,
        }
    }
}
