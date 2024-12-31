use crate::packs::metadata::pack::MetadataSection;
use crate::world::flag::feature_flag_set::FeatureFlagSet;

pub const TYPE: &str = "features";

#[derive(Debug)]
pub struct FeatureFlagsMetadataSection {
    pub flags: FeatureFlagSet,
}
impl MetadataSection for FeatureFlagsMetadataSection {}
