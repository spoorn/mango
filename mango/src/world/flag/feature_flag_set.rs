use crate::codec::Codec;
use crate::nbt::list_tag::ListTag;
use crate::nbt::tag::Tag;
use crate::resources::resource_location::ResourceLocation;
use crate::world::flag::feature_flag::FeatureFlag;
use crate::world::flag::feature_flags;
use anyhow::{anyhow, Result};
use serde::Serialize;
use tracing::error;

#[derive(Clone, Debug, Serialize)]
pub struct FeatureFlagSet {
    feature_flag_universe: String,
    mask: i64,
}
impl FeatureFlagSet {
    pub fn empty() -> Self {
        Self {
            feature_flag_universe: "".to_string(),
            mask: 0,
        }
    }

    pub fn create<'a>(universe: String, mut flags: impl Iterator<Item = &'a FeatureFlag>) -> Self {
        if flags.by_ref().peekable().peek().is_none() {
            Self::empty()
        } else {
            let mask = compute_mask(&universe, 0, flags);
            Self {
                feature_flag_universe: universe,
                mask,
            }
        }
    }

    pub fn is_subset_of(&self, feature_flag_set: &FeatureFlagSet) -> bool {
        self.feature_flag_universe == ""
            || (self.feature_flag_universe == feature_flag_set.feature_flag_universe
                && (self.mask & !feature_flag_set.mask) == 0)
    }

    fn from_names<'a>(locations: impl Iterator<Item = ResourceLocation>) -> Result<Self> {
        let mut flags = Vec::new();

        let mut errored = false;
        locations.for_each(|location| {
            // TODO: vanilla returns a DataResult.error() on missing feature Ids but still iterates
            // through features. It seems like on error, it will fallback to the default.
            // I just mark this as a TODO so it's easily searchable
            match feature_flags::FEATURE_FLAGS.registry.names.get(&location) {
                None => {
                    errored = true;
                    error!("Unknown feature id: {}", location);
                }
                Some(flag) => {
                    if !errored {
                        flags.push(flag)
                    }
                }
            }
        });

        if errored {
            Err(anyhow!("Found unknown feature Ids"))
        } else {
            Ok(Self::create(
                feature_flags::FEATURE_FLAGS.registry.universe.clone(),
                flags.into_iter(),
            ))
        }
    }
}
impl From<&FeatureFlag> for FeatureFlagSet {
    fn from(value: &FeatureFlag) -> Self {
        Self {
            feature_flag_universe: value.universe.clone(),
            mask: value.mask,
        }
    }
}
impl Default for FeatureFlagSet {
    fn default() -> Self {
        feature_flags::FEATURE_FLAGS.vanilla_set.clone()
    }
}
impl Codec for FeatureFlagSet {
    type Data = ListTag;

    fn decode(data: Self::Data) -> Result<Self> {
        let locations = data.iter().filter_map(|tag| match tag {
            Tag::StringTag(s) => Some(ResourceLocation::read(s)),
            _ => None,
        });

        Self::from_names(locations)
    }
}

fn compute_mask<'a>(
    universe: &String,
    mut mask: i64,
    flags: impl Iterator<Item = &'a FeatureFlag>,
) -> i64 {
    flags.for_each(|flag| {
        if universe != &flag.universe {
            panic!(
                "Mismatched feature universe, expected '{}', but got '{}'",
                universe, flag.universe
            );
        }

        mask |= flag.mask;
    });

    mask
}
