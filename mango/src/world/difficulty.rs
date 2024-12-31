use serde::Serialize;
use strum::FromRepr;

#[derive(Default, FromRepr, PartialEq, Debug, Serialize)]
#[repr(u8)]
pub enum Difficulty {
    Peaceful,
    Easy,
    #[default]
    Normal,
    Hard,
}
impl Difficulty {
    pub fn by_id(id: i32) -> Self {
        Self::from_repr(id as u8).unwrap_or_default()
    }
}
