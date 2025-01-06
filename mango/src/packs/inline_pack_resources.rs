use crate::file_util;
use crate::packs::abstract_pack_resources;
use crate::packs::metadata::metadata_section_type::MetadataSectionType;
use crate::packs::metadata::pack::MetadataSection;
use crate::packs::pack_location_info::PackLocationInfo;
use crate::packs::pack_resources::PackResources;
use crate::packs::pack_type::PackType;
use crate::packs::repository::pack::{Metadata, ResourcesSupplier};
use crate::resources::resource_location;
use include_dir::DirEntry;
use std::collections::HashSet;
use std::io::Read;
use std::rc::Rc;
use std::sync::Arc;
use tracing::warn;

#[derive(Debug)]
pub struct InlinePackResourceSupplier {
    pub entry: &'static DirEntry<'static>,
}
impl ResourcesSupplier for InlinePackResourceSupplier {
    fn open_primary(&self, location: PackLocationInfo) -> Arc<dyn PackResources> {
        Arc::new(InlinePackResources {
            location,
            entry: self.entry,
        })
    }

    fn open_full(
        &self,
        location: PackLocationInfo,
        _metadata: &Metadata,
    ) -> Arc<dyn PackResources> {
        // Same as open_primary as there can be no nested resources for InlinePackResources
        Arc::new(InlinePackResources {
            location,
            entry: self.entry,
        })
    }
}

/// Resource Supplier that outputs the file contents directly
pub struct InlinePackResources {
    pub location: PackLocationInfo,
    /// Either a directory, or a zip file. Non zip files will fail
    pub entry: &'static DirEntry<'static>,
}
impl PackResources for InlinePackResources {
    fn get_root_resource(&self, paths: &[&str]) -> Option<Box<dyn Read>> {
        file_util::validate_path(paths).ok()?;
        let path = file_util::resolve_path(paths);
        match self.entry {
            DirEntry::Dir(dir) => dir
                .get_file(dir.path().join(path))
                .map(|e| Box::new(e.contents()) as _),
            DirEntry::File(_file) => todo!("inline zip files not yet supported"),
        }
    }

    fn get_namespaces(&self, pack_type: PackType) -> HashSet<String> {
        match self.entry {
            DirEntry::Dir(root_dir) => {
                let mut res = HashSet::new();
                if let Some(pack_dir) =
                    root_dir.get_dir(root_dir.path().join(pack_type.get_directory()))
                {
                    pack_dir.dirs().for_each(|dir| {
                        let namespace = dir.path().file_name().unwrap().to_str().unwrap();
                        if resource_location::is_valid_namespace(&namespace) {
                            res.insert(namespace.to_string());
                        } else {
                            warn!(
                                "Non [a-z0-9_.-] character in namespace {} in pack {}, ignoring",
                                namespace,
                                root_dir.path().display()
                            );
                        }
                    })
                }
                res
            }
            DirEntry::File(_file) => todo!("inline zip files not yet supported"),
        }
    }

    fn get_metadata_section(
        &self,
        metadata_section_type: MetadataSectionType,
    ) -> Option<Rc<dyn MetadataSection>> {
        let root_resource = self.get_root_resource(&["pack.mcmeta"])?;
        abstract_pack_resources::get_metadata_from_stream(&metadata_section_type, root_resource)
            .map(Rc::from)
    }

    fn location(&self) -> &PackLocationInfo {
        &self.location
    }
}
