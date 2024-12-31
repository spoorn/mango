use serde::Serialize;
use strum::FromRepr;

#[derive(Default, FromRepr, PartialEq, Debug, Serialize)]
#[repr(u8)]
pub enum GameType {
    #[default]
    Survival,
    Creative,
    Adventure,
    Spectator,
}
impl GameType {
    pub fn by_id(id: i32) -> Self {
        Self::from_repr(id as u8).unwrap_or_default()
    }
}
