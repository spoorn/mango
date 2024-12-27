use crate::world::entity::entity_attachment::EntityAttachment;
use crate::world::phys::vec3::Vec3;
use serde::Serialize;
use std::collections::HashMap;
use strum::IntoEnumIterator;

#[derive(Debug, Serialize)]
pub struct EntityAttachments {
    attachments: HashMap<EntityAttachment, Vec<Vec3>>,
}
impl EntityAttachments {
    pub fn new() -> Self {
        Self {
            attachments: HashMap::new(),
        }
    }

    pub fn attach(&mut self, attachment: EntityAttachment, vec3: Vec3) {
        self.attachments
            .entry(attachment)
            .or_insert_with(|| Vec::with_capacity(1))
            .push(vec3);
    }

    pub fn create_default(width: f32, height: f32) -> Self {
        Self {
            attachments: EntityAttachment::iter()
                .map(|e| {
                    let fallback_points = e.create_fallback_points(width as f64, height as f64);
                    (e, fallback_points)
                })
                .collect::<HashMap<EntityAttachment, Vec<Vec3>>>(),
        }
    }
}
