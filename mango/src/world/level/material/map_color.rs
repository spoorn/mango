pub const NONE: MapColor = MapColor::new(0, 0);
pub const FIRE: MapColor = MapColor::new(4, 16711680);

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
