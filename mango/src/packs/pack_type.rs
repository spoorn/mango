use include_dir::{include_dir, Dir};
use strum::EnumIter;

pub const MC_ASSETS_ROOT_FILE: &str = ".mcassetsroot";

pub static ASSETS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/../assets");
pub static DATA_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/../data");

#[derive(Copy, Clone, EnumIter, Eq, Hash, PartialEq, Debug)]
pub enum PackType {
    ClientResources,
    ServerData,
}
impl PackType {
    pub fn get_directory(&self) -> &Dir<'static> {
        match self {
            PackType::ClientResources => &ASSETS_DIR,
            PackType::ServerData => &DATA_DIR,
        }
    }
}
