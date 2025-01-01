use std::ops::RangeInclusive;

#[derive(Clone, Debug)]
pub enum PackCompatibility {
    TooOld,
    TooNew,
    Compatible,
}
impl PackCompatibility {
    pub fn for_version(range: RangeInclusive<u32>, pack_version: u32) -> Self {
        if *range.end() < pack_version {
            PackCompatibility::TooOld
        } else if pack_version < *range.start() {
            PackCompatibility::TooNew
        } else {
            PackCompatibility::Compatible
        }
    }
}
