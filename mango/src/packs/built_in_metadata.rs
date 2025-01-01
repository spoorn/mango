use crate::packs::metadata::metadata_section_type::MetadataSectionType;
use crate::packs::metadata::pack::MetadataSection;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Default, Debug)]
pub struct BuiltInMetadata {
    values: HashMap<MetadataSectionType, Rc<dyn MetadataSection>>,
}
impl BuiltInMetadata {
    pub fn of(
        key_values: impl Iterator<Item = (MetadataSectionType, Rc<dyn MetadataSection>)>,
    ) -> Self {
        Self {
            values: HashMap::from_iter(key_values),
        }
    }
}
impl Deref for BuiltInMetadata {
    type Target = HashMap<MetadataSectionType, Rc<dyn MetadataSection>>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}
