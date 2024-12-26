use crate::resources::resource_location::ResourceLocation;

#[derive(Debug, Clone)]
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
