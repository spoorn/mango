use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum LevelResource {
    LevelDataFile,
    OldLevelDataFile,
    IconFile,
    DatapackDir,
}
impl LevelResource {
    pub const fn id(&self) -> &str {
        match self {
            LevelResource::LevelDataFile => "level.dat",
            LevelResource::OldLevelDataFile => "level.dat_old",
            LevelResource::IconFile => "icon.png",
            LevelResource::DatapackDir => "datapacks",
        }
    }
}
impl Display for LevelResource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "/{}", self.id())
    }
}
