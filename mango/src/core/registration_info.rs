use crate::core::mapped_registry::Lifecycle;
use crate::packs::repository::known_pack::KnownPack;

pub const BUILT_IN: RegistrationInfo = RegistrationInfo {
    known_pack_info: None,
    lifecycle: Lifecycle::Stable,
};

pub struct RegistrationInfo {
    pub known_pack_info: Option<KnownPack>,
    pub lifecycle: Lifecycle,
}
