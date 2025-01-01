use crate::packs::repository::pack::Position;

#[derive(Clone, Debug)]
pub struct PackSelectionConfig {
    pub required: bool,
    pub default_position: Position,
    pub fixed_position: bool,
}
