pub const NONE: MapColor = MapColor::new(0, 0);
pub const WOOL: MapColor = MapColor::new(3, 13092807);
pub const FIRE: MapColor = MapColor::new(4, 16711680);
pub const PLANT: MapColor = MapColor::new(7, 31744);
pub const WOOD: MapColor = MapColor::new(13, 9402184);

#[derive(Debug)]
pub struct MapColor {
    pub id: i32,
    pub col: i32,
}

impl MapColor {
    const fn new(id: i32, col: i32) -> Self {
        Self { id, col }
    }
}
