use crate::codec::Codec;
use crate::nbt::compound_tag::CompoundTag;
use crate::util::datafix::serialization::dynamic::Dynamic;
use crate::world::flag::feature_flag_set::FeatureFlagSet;
use crate::world::level::data_pack_config::DataPackConfig;
use serde::Serialize;
use std::borrow::Borrow;

#[derive(Debug, Default, Serialize)]
pub struct WorldDataConfiguration {
    datapacks: DataPackConfig,
    pub enabled_features: FeatureFlagSet,
}
impl WorldDataConfiguration {
    pub fn new(datapacks: DataPackConfig, enabled_features: FeatureFlagSet) -> Self {
        Self {
            datapacks,
            enabled_features,
        }
    }
}
impl Codec<Dynamic<CompoundTag>> for WorldDataConfiguration {
    fn decode(data: Dynamic<CompoundTag>) -> anyhow::Result<Self> {
        let data = data.borrow();
        let datapacks = data
            .value
            .try_get_compound("DataPacks")
            .map_or(Ok(DataPackConfig::default()), DataPackConfig::decode)?;

        let enabled_features = data
            .value
            .try_get_list("enabled_features")
            .map_or(Ok(FeatureFlagSet::default()), FeatureFlagSet::decode)?;

        Ok(Self {
            datapacks,
            enabled_features,
        })
    }
}
