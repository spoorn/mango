use crate::packs::file_pack_resources::FileResourcesSupplier;
use crate::packs::inline_pack_resources::InlinePackResourceSupplier;
use crate::packs::repository::pack::ResourcesSupplier;
use include_dir::DirEntry;
use std::rc::Rc;

/// This differs quite a bit from vanilla as we inlined the resources into the binary.
/// Thus, this does not support symlinks and does not need to run the DirectoryValidator
pub fn detect_pack_resources(
    entry: &'static include_dir::DirEntry,
) -> Option<Rc<dyn ResourcesSupplier>> {
    match entry {
        DirEntry::Dir(dir) => match dir.get_file("pack.mcmeta") {
            None => None,
            Some(mcmeta_file) => Some(Rc::new(InlinePackResourceSupplier {
                contents: mcmeta_file.contents(),
            })),
        },
        DirEntry::File(file) => {
            if file.path().ends_with(".zip") {
                // Inline file can technically be a zip file
                Some(Rc::new(FileResourcesSupplier::new(file.contents())))
            } else {
                None
            }
        }
    }
}
