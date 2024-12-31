use std::path::PathBuf;

#[derive(Debug)]
pub struct ForbiddenSymlinkInfo {
    pub link: PathBuf,
    pub target: PathBuf,
}
