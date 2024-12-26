use crate::core::registration_info;
use crate::core::registration_info::RegistrationInfo;
use crate::core::registries::registries;
use crate::core::registries::registries::root_registry_name;
use crate::resources::resource_key::ResourceKey;
use crate::resources::resource_location::ResourceLocation;
use crate::world::level::block::block::BlockTrait;
use dashmap::DashMap;
use std::fmt::Debug;
use std::sync::{Arc, OnceLock, RwLock};

// TODO: Optimize and remove RwLock?
pub static REGISTRY: OnceLock<MappedRegistry<Arc<dyn Registry>>> = OnceLock::new();

pub static BLOCK: OnceLock<Arc<MappedRegistry<Arc<dyn BlockTrait>>>> = OnceLock::new();

pub fn registry() -> &'static MappedRegistry<Arc<dyn Registry>> {
    REGISTRY.get().unwrap()
}

pub fn block_registry() -> Arc<MappedRegistry<Arc<dyn BlockTrait>>> {
    Arc::clone(BLOCK.get().unwrap())
}

pub fn bootstrap() {
    REGISTRY.get_or_init(|| {
        MappedRegistry::new(
            ResourceKey::create_registry_key(root_registry_name()),
            Lifecycle::Stable,
        )
    });
    BLOCK.get_or_init(|| register_defaulted_with_intrusive_holders(registries::BLOCK.clone()));
}

#[derive(Debug)]
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
}

#[derive(Debug)]
pub struct MappedRegistry<T> {
    pub key: ResourceKey,
    pub lifecycle: Lifecycle,
    values: RwLock<Vec<T>>,
    by_key: DashMap<ResourceKey, usize>,
    by_location: DashMap<ResourceLocation, usize>,
    frozen: bool,
}

impl<T> MappedRegistry<T> {
    pub fn new(key: ResourceKey, lifecycle: Lifecycle) -> Self {
        Self {
            key,
            lifecycle,
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
        // let value_rg = self.values.read().unwrap();
        // value_rg.get(index).unwrap_or_else(|| {
        //     panic!(
        //         "Failed to get value at index {} from Mapped Registry with key {}",
        //         index, self.key
        //     )
        // })
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
}

// TODO: No DefaultedMappedRegistry
fn register_defaulted_with_intrusive_holders<T: Send + Sync + Debug + 'static>(
    key: ResourceKey,
) -> Arc<MappedRegistry<T>> {
    internal_register(key.clone(), MappedRegistry::new(key, Lifecycle::Stable))
}

fn internal_register<R: Registry + Send + Sync + 'static>(key: ResourceKey, value: R) -> Arc<R> {
    let arc_value = Arc::new(value);
    registry().register(key, arc_value.clone(), registration_info::BUILT_IN);
    arc_value
}
