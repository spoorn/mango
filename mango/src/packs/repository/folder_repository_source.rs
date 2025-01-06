use crate::packs::pack_type::PackType;
use crate::packs::repository::pack::{Pack, ResourcesSupplier};
use crate::packs::repository::pack_detector;
use crate::packs::repository::pack_source::PackSource;
use crate::packs::repository::repository_source::RepositorySource;
use crate::world::level::validation::directory_validator::DirectoryValidator;
use std::path::PathBuf;
use std::rc::Rc;
use tracing::{info, warn};

#[derive(Debug)]
pub struct FolderRepositorySource {
    folder: PathBuf,
    pack_type: PackType,
    pack_source: PackSource,
    validator: DirectoryValidator,
}
impl FolderRepositorySource {
    pub fn new(
        folder: PathBuf,
        pack_type: PackType,
        pack_source: PackSource,
        validator: DirectoryValidator,
    ) -> Self {
        Self {
            folder,
            pack_type,
            pack_source,
            validator,
        }
    }
}
impl RepositorySource for FolderRepositorySource {
    fn load_packs(&self, consumer: &mut dyn FnMut(Pack) -> ()) {
        // TODO: implement this
    }
}

pub fn discover_packs(
    dir: &'static include_dir::Dir,
    _validator: DirectoryValidator,
    mut consumer: impl FnMut(&'static include_dir::DirEntry, Rc<dyn ResourcesSupplier>) -> (),
) {
    dir.entries().iter().for_each(|entry| {
        match pack_detector::detect_inlined_pack_resources(entry) {
            None => warn!(
                "Found non-pack entry '{}', ignoring",
                entry.path().display()
            ),
            Some(supplier) => {
                info!("Discovered pack entry '{}'", entry.path().display());
                consumer(entry, supplier)
            }
        }
    });
}
