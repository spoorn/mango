use crate::world::flag::feature_flag::FeatureFlag;
use crate::world::flag::feature_flag_registry::{FeatureFlagRegistry, FeatureFlagRegistryBuilder};
use crate::world::flag::feature_flag_set::FeatureFlagSet;
use std::sync::LazyLock;

pub static FEATURE_FLAGS: LazyLock<FeatureFlags> = LazyLock::new(|| {
    let mut registry_builder = FeatureFlagRegistryBuilder::default();
    let vanilla = registry_builder.create_vanilla("vanilla");
    let trade_rebalance = registry_builder.create_vanilla("trade_rebalance");
    let redstone_experiments = registry_builder.create_vanilla("redstone_experiments");
    let minecart_improvements = registry_builder.create_vanilla("minecart_improvements");
    let registry = registry_builder.build();
    let vanilla_set = FeatureFlagSet::from(&vanilla);
    let default_flags = vanilla_set.clone();
    FeatureFlags {
        vanilla,
        trade_rebalance,
        redstone_experiments,
        minecart_improvements,
        registry,
        vanilla_set,
        default_flags,
    }
});

pub struct FeatureFlags {
    pub vanilla: FeatureFlag,
    pub trade_rebalance: FeatureFlag,
    pub redstone_experiments: FeatureFlag,
    pub minecart_improvements: FeatureFlag,
    pub registry: FeatureFlagRegistry,
    pub vanilla_set: FeatureFlagSet,
    pub default_flags: FeatureFlagSet,
}

pub fn is_experimental(feature_flags: &FeatureFlagSet) -> bool {
    !feature_flags.is_subset_of(&FEATURE_FLAGS.vanilla_set)
}
