use crate::network::chat::mutable_component::MutableComponent;
use crate::packs::repository::known_pack::KnownPack;
use crate::packs::repository::pack_source::PackSource;

#[derive(Debug)]
pub struct PackLocationInfo {
    id: String,
    title: MutableComponent,
    source: PackSource,
    known_pack_info: Option<KnownPack>,
}
impl PackLocationInfo {
    pub fn new(
        id: String,
        title: MutableComponent,
        source: PackSource,
        known_pack_info: Option<KnownPack>,
    ) -> Self {
        Self {
            id,
            title,
            source,
            known_pack_info,
        }
    }
}
