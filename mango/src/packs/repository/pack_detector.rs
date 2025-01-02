use crate::packs::inline_pack_resources::InlinePackResourceSupplier;
use crate::packs::repository::pack::ResourcesSupplier;
use include_dir::DirEntry;
use std::rc::Rc;
use tracing::debug;

/// This differs quite a bit from vanilla as we inlined the resources into the binary.
/// Thus, this does not support symlinks and does not need to run the DirectoryValidator
pub fn detect_pack_resources(
    entry: &'static include_dir::DirEntry,
) -> Option<Rc<dyn ResourcesSupplier>> {
    match entry {
        // include_dirs is a little clunky and requires the full path
        DirEntry::Dir(dir) => match dir.get_file(dir.path().join("pack.mcmeta")) {
            None => {
                debug!(
                    "Ignoring directory '{}', no pack.mcmeta found",
                    dir.path().display()
                );
                None
            }
            Some(mcmeta_file) => Some(Rc::new(InlinePackResourceSupplier {
                contents: mcmeta_file.contents(),
            })),
        },
        DirEntry::File(file) => {
            if file.path().ends_with(".zip") {
                // Inline file can technically be a zip file but there doesn't seem to be any yet
                todo!("Zip files are not supported yet for built in packs");
                //Some(Rc::new(FileResourcesSupplier::new(file.contents())))
            } else {
                debug!("Ignoring non-pack file '{}'", file.path().display());
                None
            }
        }
    }
}
