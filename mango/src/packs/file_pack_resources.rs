use crate::packs::metadata::metadata_section_type::MetadataSectionType;
use crate::packs::metadata::pack::MetadataSection;
use crate::packs::pack_location_info::PackLocationInfo;
use crate::packs::pack_resources::PackResources;
use crate::packs::repository::pack::{Metadata, ResourcesSupplier};
use std::fmt::Debug;
use std::io::Read;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug)]
pub struct FileResourcesSupplier<T: Read + Debug> {
    content: T,
}
impl<T: Read + Debug> FileResourcesSupplier<T> {
    pub fn new(content: T) -> Self {
        Self { content }
    }
}
/// For inline data
// impl<T: Read + Debug> From<&[u8]> for FileResourcesSupplier<T> {
//     fn from(value: &[u8]) -> Self {
//         Self { content: value }
//     }
// }
impl<T: Read + Debug> ResourcesSupplier for FileResourcesSupplier<T> {
    fn open_primary(&self, location: &PackLocationInfo) -> Arc<dyn PackResources> {
        todo!()
    }

    fn open_full(
        &self,
        location: &PackLocationInfo,
        metadata: &Metadata,
    ) -> Arc<dyn PackResources> {
        todo!()
    }
}

pub struct FilePackResources {}
impl PackResources for FilePackResources {
    fn get_root_resource(&self, paths: &[&str]) -> Option<&[u8]> {
        todo!()
    }

    fn get_metadata_section(
        &self,
        metadata_section_type: MetadataSectionType,
    ) -> Option<Rc<dyn MetadataSection>> {
        todo!()
    }
}
