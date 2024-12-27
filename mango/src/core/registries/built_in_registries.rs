use crate::core::mapped_registry::{Lifecycle, MappedRegistry, Registry, WritableRegistry};
use crate::core::registration_info;
use crate::core::registries::registries;
use crate::core::registries::registries::root_registry_name;
use crate::resources::resource_key::ResourceKey;
use crate::sounds::sound_event::SoundEvent;
use crate::world::entity::entity_type::EntityType;
use crate::world::item::item::ItemTrait;
use crate::world::level::block::block::BlockTrait;
use std::fmt::Debug;
use std::sync::{Arc, OnceLock};

pub static REGISTRY: OnceLock<MappedRegistry<Arc<dyn Registry>>> = OnceLock::new();
pub static BLOCK: OnceLock<Arc<MappedRegistry<Arc<dyn BlockTrait>>>> = OnceLock::new();
pub static ITEM: OnceLock<Arc<MappedRegistry<Arc<dyn ItemTrait>>>> = OnceLock::new();
pub static ENTITY_TYPE: OnceLock<Arc<MappedRegistry<Arc<EntityType>>>> = OnceLock::new();
// TODO: does SoundEvent need to be wrapped around Arc? They seem to be immutable so maybe we just copy everywhere
pub static SOUND_EVENT: OnceLock<Arc<MappedRegistry<SoundEvent>>> = OnceLock::new();

pub fn registry() -> &'static MappedRegistry<Arc<dyn Registry>> {
    REGISTRY.get().unwrap()
}

pub fn block_registry() -> Arc<MappedRegistry<Arc<dyn BlockTrait>>> {
    Arc::clone(BLOCK.get().unwrap())
}

pub fn get_block(id: usize) -> Option<Arc<dyn BlockTrait>> {
    (block_registry()
        as Arc<dyn WritableRegistry<Arc<dyn BlockTrait>, Result = Arc<dyn BlockTrait>>>)
        .get(id)
}

pub fn item_registry() -> Arc<MappedRegistry<Arc<dyn ItemTrait>>> {
    Arc::clone(ITEM.get().unwrap())
}

pub fn entity_type_registry() -> Arc<MappedRegistry<Arc<EntityType>>> {
    Arc::clone(ENTITY_TYPE.get().unwrap())
}

pub fn sound_event_registry() -> Arc<MappedRegistry<SoundEvent>> {
    Arc::clone(SOUND_EVENT.get().unwrap())
}

pub fn bootstrap() {
    REGISTRY.get_or_init(|| {
        MappedRegistry::new(
            ResourceKey::create_registry_key(root_registry_name()),
            Lifecycle::Stable,
        )
    });
    BLOCK.get_or_init(|| register_defaulted_with_intrusive_holders(registries::BLOCK.clone()));
    ITEM.get_or_init(|| register_defaulted_with_intrusive_holders(registries::ITEM.clone()));
    ENTITY_TYPE
        .get_or_init(|| register_defaulted_with_intrusive_holders(registries::ENTITY_TYPE.clone()));
    SOUND_EVENT.get_or_init(|| register_simple(registries::SOUND_EVENT.clone()));
}

fn register_simple<T: Send + Sync + Debug + 'static>(key: ResourceKey) -> Arc<MappedRegistry<T>> {
    internal_register(key.clone(), MappedRegistry::new(key, Lifecycle::Stable))
}

// TODO: No DefaultedMappedRegistry so this is the same as register_simple
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
