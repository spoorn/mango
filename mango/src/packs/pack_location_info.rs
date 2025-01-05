use crate::network::chat::mutable_component::MutableComponent;
use crate::packs::repository::known_pack::KnownPack;
use crate::packs::repository::pack_source::PackSource;

#[derive(Clone, Debug)]
pub struct PackLocationInfo {
    pub id: String,
    title: MutableComponent,
    pub source: PackSource,
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
/// We ignore checking equality for the [MutableComponent] as it gets a little messy dealing with
/// trait objects and implementing PartialEq for it.
/// The hope here is that if the other fields are equal, we should consider the locations equal.
impl PartialEq for PackLocationInfo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.source == other.source
            && self.known_pack_info == other.known_pack_info
    }
}
