use crate::resources::resource_location::ResourceLocation;
use crate::world::flag::feature_flag::FeatureFlag;
use crate::world::flag::feature_flag_set::FeatureFlagSet;
use std::collections::HashMap;

#[derive(Debug)]
pub struct FeatureFlagRegistry {
    pub universe: String,
    pub names: HashMap<ResourceLocation, FeatureFlag>,
    all_flags: FeatureFlagSet,
}

pub struct FeatureFlagRegistryBuilder {
    universe: String,
    id: u8,
    flags: HashMap<ResourceLocation, FeatureFlag>,
}
impl FeatureFlagRegistryBuilder {
    pub fn create_vanilla(&mut self, path: &str) -> FeatureFlag {
        self.create(ResourceLocation::with_default_namespace(path))
    }

    pub fn create(&mut self, location: ResourceLocation) -> FeatureFlag {
        if self.id > 63 {
            panic!("Too many feature flags");
        }
        let flag = FeatureFlag::new(self.universe.clone(), self.id);
        self.id += 1;
        if self.flags.contains_key(&location) {
            panic!(
                "Duplicate feature flag for location {}: {:?}",
                location, flag
            );
        }
        self.flags.insert(location.clone(), flag.clone());
        flag
    }

    pub fn build(self) -> FeatureFlagRegistry {
        let all_flags = FeatureFlagSet::create(self.universe.clone(), self.flags.values());
        FeatureFlagRegistry {
            universe: self.universe,
            names: self.flags,
            all_flags,
        }
    }
}
impl Default for FeatureFlagRegistryBuilder {
    fn default() -> Self {
        Self {
            universe: "main".to_string(),
            id: 0,
            flags: HashMap::new(),
        }
    }
}
