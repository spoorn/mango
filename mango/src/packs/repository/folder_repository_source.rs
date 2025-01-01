use crate::packs::pack_type::PackType;
use crate::packs::repository::pack::Pack;
use crate::packs::repository::pack_source::PackSource;
use crate::packs::repository::repository_source::RepositorySource;
use crate::world::level::validation::directory_validator::DirectoryValidator;
use std::path::PathBuf;

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
    fn load_packs(&self, consumer: &dyn FnOnce(Pack) -> ()) {
        todo!()
    }
}
