use crate::packs::metadata::metadata_section_type::MetadataSectionType;
use crate::packs::metadata::pack::MetadataSection;
use std::rc::Rc;

pub trait PackResources {
    // TODO: Make this a stream
    fn get_root_resource(&self, paths: &[&str]) -> Option<&[u8]>;

    fn get_metadata_section(
        &self,
        metadata_section_type: MetadataSectionType,
    ) -> Option<Rc<dyn MetadataSection>>;
}
