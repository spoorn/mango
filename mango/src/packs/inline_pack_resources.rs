use crate::packs::abstract_pack_resources;
use crate::packs::metadata::metadata_section_type::MetadataSectionType;
use crate::packs::metadata::pack::MetadataSection;
use crate::packs::pack_location_info::PackLocationInfo;
use crate::packs::pack_resources::PackResources;
use crate::packs::repository::pack::{Metadata, ResourcesSupplier};
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug)]
pub struct InlinePackResourceSupplier {
    pub contents: &'static [u8],
}
impl ResourcesSupplier for InlinePackResourceSupplier {
    fn open_primary(&self, _location: &PackLocationInfo) -> Arc<dyn PackResources> {
        Arc::new(InlinePackResources {
            contents: self.contents,
        })
    }

    fn open_full(
        &self,
        _location: &PackLocationInfo,
        _metadata: &Metadata,
    ) -> Arc<dyn PackResources> {
        // Same as open_primary as there can be no nested resources for InlinePackResources
        Arc::new(InlinePackResources {
            contents: self.contents,
        })
    }
}

/// Resource Supplier that outputs the file contents directly
pub struct InlinePackResources {
    pub contents: &'static [u8],
}
impl PackResources for InlinePackResources {
    fn get_root_resource(&self, _paths: &[&str]) -> Option<&[u8]> {
        Some(self.contents)
    }

    fn get_metadata_section(
        &self,
        metadata_section_type: MetadataSectionType,
    ) -> Option<Rc<dyn MetadataSection>> {
        let root_resource = self.get_root_resource(&["pack.mcmeta"])?;
        abstract_pack_resources::get_metadata_from_stream(&metadata_section_type, root_resource)
            .map(Rc::from)
    }
}
