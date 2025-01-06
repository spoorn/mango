use crate::packs::metadata::metadata_section_type::MetadataSectionType;
use crate::packs::metadata::pack::MetadataSection;
use crate::packs::pack_location_info::PackLocationInfo;
use crate::packs::pack_type::PackType;
use std::collections::HashSet;
use std::io::Read;
use std::rc::Rc;

/// Pack Resources are currently fully synchronous as we read pack data in synchronous functions
/// due to limitations such as serde json not supporting async: https://github.com/serde-rs/json/issues/575
/// This should be fine as reloading of pack resources should only happen during startup or reload
/// commands.
///
/// Datapack format: https://minecraft.fandom.com/wiki/Data_pack
pub trait PackResources {
    fn get_root_resource(&self, paths: &[&str]) -> Option<Box<dyn Read>>;

    fn get_namespaces(&self, pack_type: PackType) -> HashSet<String>;

    fn get_metadata_section(
        &self,
        metadata_section_type: MetadataSectionType,
    ) -> Option<Rc<dyn MetadataSection>>;

    fn location(&self) -> &PackLocationInfo;

    fn pack_id(&self) -> &String {
        &self.location().id
    }
}
