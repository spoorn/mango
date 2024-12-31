use std::fmt::{Display, Formatter};

pub const LEVEL_DATA_FILE: LevelResource = LevelResource::new("level.dat");
pub const OLD_LEVEL_DATA_FILE: LevelResource = LevelResource::new("level.dat_old");
pub const ICON_FILE: LevelResource = LevelResource::new("icon.png");

pub struct LevelResource {
    pub id: &'static str,
}
impl LevelResource {
    const fn new(id: &'static str) -> Self {
        Self { id }
    }
}
impl Display for LevelResource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "/{}", self.id)
    }
}
