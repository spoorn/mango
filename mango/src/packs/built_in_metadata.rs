use crate::packs::metadata::pack::MetadataSection;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct BuiltInMetadata {
    values: HashMap<String, Box<dyn MetadataSection>>,
}
impl BuiltInMetadata {
    pub fn of(key_values: impl Iterator<Item = (String, Box<dyn MetadataSection>)>) -> Self {
        Self {
            values: HashMap::from_iter(key_values),
        }
    }
}
