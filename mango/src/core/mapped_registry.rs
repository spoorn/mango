use crate::core::registration_info::RegistrationInfo;
use crate::resources::resource_key::ResourceKey;
use crate::resources::resource_location::ResourceLocation;
use crate::world::level::block::block::BlockTrait;
use dashmap::DashMap;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Copy)]
pub enum Lifecycle {
    Stable,
    Experimental,
    Deprecated { since: i32 },
}

impl Lifecycle {
    pub fn add(self, other: Lifecycle) -> Self {
        match (self, other) {
            (Lifecycle::Stable, Lifecycle::Stable) => Lifecycle::Stable,
            (Lifecycle::Experimental, _) => Lifecycle::Experimental,
            (_, Lifecycle::Experimental) => Lifecycle::Experimental,
            (Lifecycle::Deprecated { since: a }, Lifecycle::Deprecated { since: b }) => {
                Lifecycle::Deprecated { since: a.min(b) }
            }
            (Lifecycle::Deprecated { since: a }, _) => Lifecycle::Deprecated { since: a },
            (_, Lifecycle::Deprecated { since: b }) => Lifecycle::Deprecated { since: b },
        }
    }
}

pub trait Registry: Send + Sync + Debug {}

pub trait WritableRegistry<T> {
    fn register(&self, key: ResourceKey, value: T, registration_info: RegistrationInfo);

    fn key(&self) -> &ResourceKey;
}

/// Registries can be accessed via static reference by anywhere in the code, thus everything here is
/// made thread-safe.
///
/// If performance becomes a concern, we can switch to unsafe without significant redesign.
#[derive(Debug)]
pub struct MappedRegistry<T> {
    pub key: ResourceKey,
    pub lifecycle: RwLock<Lifecycle>,
    values: RwLock<Vec<T>>,
    by_key: DashMap<ResourceKey, usize>,
    by_location: DashMap<ResourceLocation, usize>,
    frozen: bool,
}

impl<T> MappedRegistry<T> {
    pub fn new(key: ResourceKey, lifecycle: Lifecycle) -> Self {
        Self {
            key,
            lifecycle: RwLock::new(lifecycle),
            values: RwLock::new(Vec::new()),
            by_key: DashMap::new(),
            by_location: DashMap::new(),
            frozen: false,
        }
    }

    pub fn is_key_set_empty(&self) -> bool {
        self.by_location.is_empty()
    }

    fn validate_write(&self, key: &ResourceKey) {
        if self.frozen {
            panic!("Registry is already frozen (trying to add key {})", key);
        }
    }
}

impl<T: Send + Sync + Debug> Registry for MappedRegistry<T> {}

/// https://www.reddit.com/r/rust/comments/droxdg/why_arent_traits_impld_for_boxdyn_trait/
impl<T> WritableRegistry<T> for MappedRegistry<T> {
    fn register(&self, key: ResourceKey, value: T, registration_info: RegistrationInfo) {
        self.validate_write(&key);
        if self.by_location.contains_key(&key.location) {
            panic!("Adding duplicate key '{}' to registry", key);
        }
        // TODO: other checks
        let index = {
            let mut values = self.values.write().unwrap();
            let index = values.len();
            values.push(value);
            index
        };
        self.by_location.insert(key.location.clone(), index);
        self.by_key.insert(key, index);
        let mut lifecycle = self.lifecycle.write().unwrap();
        *lifecycle = lifecycle.add(registration_info.lifecycle);
        // let value_rg = self.values.read().unwrap();
        // value_rg.get(index).unwrap_or_else(|| {
        //     panic!(
        //         "Failed to get value at index {} from Mapped Registry with key {}",
        //         index, self.key
        //     )
        // })
    }

    fn key(&self) -> &ResourceKey {
        &self.key
    }
}

/// This is needed because Arc<dyn Trait> is not implemented for Arc<T: Trait> from above with
/// generic args - no automatic coercion. We have to implement it explicitly and delegate to the
/// default impl by casting to the trait with the same trait object generic arg.
///
/// https://stackoverflow.com/questions/78635767/understanding-impl-dyn-trait
/// https://users.rust-lang.org/t/help-with-not-satisfied-trait-with-box-and-dyn/77019/2
/// https://www.reddit.com/r/rust/comments/droxdg/why_arent_traits_impld_for_boxdyn_trait/
///
/// This allows callers to use trait generics instead of trait objects. For example,
/// see [crate::core::registry::register_key].
impl<T: BlockTrait + 'static> WritableRegistry<Arc<T>> for MappedRegistry<Arc<dyn BlockTrait>> {
    fn register(&self, key: ResourceKey, value: Arc<T>, registration_info: RegistrationInfo) {
        (self as &dyn WritableRegistry<Arc<dyn BlockTrait>>).register(
            key,
            value,
            registration_info,
        );
    }

    fn key(&self) -> &ResourceKey {
        (self as &dyn WritableRegistry<Arc<dyn BlockTrait>>).key()
    }
}
