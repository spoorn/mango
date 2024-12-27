use crate::world::entity::entity_attachment::EntityAttachment;
use crate::world::entity::entity_attachments::EntityAttachments;
use crate::world::phys::vec3::Vec3;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct EntityDimensions {
    pub width: f32,
    pub height: f32,
    pub eye_height: f32,
    pub attachments: EntityAttachments,
    pub fixed: bool,
}
impl EntityDimensions {
    fn new(width: f32, height: f32, fixed: bool) -> Self {
        Self {
            width,
            height,
            // Default eye height is 85% of the total height
            eye_height: height * 0.85,
            attachments: EntityAttachments::create_default(width, height),
            fixed,
        }
    }

    pub fn attach(&mut self, attachment: EntityAttachment, vec3: Vec3) {
        self.attachments.attach(attachment, vec3);
    }

    pub fn scalable(width: f32, height: f32) -> Self {
        Self::new(width, height, false)
    }
}
