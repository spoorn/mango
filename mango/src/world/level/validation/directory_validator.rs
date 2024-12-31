use crate::world::level::validation::forbidden_symlink_info::ForbiddenSymlinkInfo;
use std::path::PathBuf;
use walkdir::WalkDir;

type PathMatcher = fn(&PathBuf) -> bool;

pub struct DirectoryValidator {
    symlink_target_allowlist: PathMatcher,
}
impl DirectoryValidator {
    pub fn new(symlink_target_allowlist: PathMatcher) -> Self {
        Self {
            symlink_target_allowlist,
        }
    }

    pub fn validate_symlink(&self, path: PathBuf, errors: &mut Vec<ForbiddenSymlinkInfo>) {
        let target = std::fs::read_link(&path).expect("Failed to read symlink target");
        if !(self.symlink_target_allowlist)(&target) {
            errors.push(ForbiddenSymlinkInfo { link: path, target });
        }
    }

    pub fn validate_directory(
        &self,
        mut path: PathBuf,
        skip_symlink_validation: bool,
    ) -> Vec<ForbiddenSymlinkInfo> {
        // Get file metadata without following symlinks
        let metadata = match std::fs::symlink_metadata(&path) {
            Ok(m) => m,
            Err(_e) => return Vec::new(),
        };

        if metadata.is_file() {
            panic!("Path {:?} is not a directory", path);
        } else {
            let mut res = Vec::new();
            if metadata.is_symlink() {
                if !skip_symlink_validation {
                    self.validate_symlink(path, &mut res);
                    return res;
                }

                let symlink_target =
                    std::fs::read_link(&path).expect("Failed to read symlink target");
                let _ = std::mem::replace(&mut path, symlink_target);
            }

            self.validate_known_directory(path, &mut res);
            res
        }
    }

    /// Verifies all nested symlinks
    pub fn validate_known_directory(&self, path: PathBuf, errors: &mut Vec<ForbiddenSymlinkInfo>) {
        WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_symlink())
            .for_each(|e| self.validate_symlink(e.path().to_path_buf(), errors));
    }
}
