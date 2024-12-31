/// Represents a single feature flag as a single flipped bit in an i64
#[derive(Clone, Debug)]
pub struct FeatureFlag {
    pub universe: String,
    pub mask: i64,
}
impl FeatureFlag {
    pub fn new(universe: String, bit_flag: u8) -> Self {
        Self {
            universe,
            mask: 1 << bit_flag,
        }
    }
}
