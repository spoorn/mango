use crate::network::chat::mutable_component::MutableComponent;
use crate::packs::pack_location_info::PackLocationInfo;
use crate::packs::pack_resources::PackResources;
use crate::packs::pack_selection_config::PackSelectionConfig;
use crate::packs::pack_type::PackType;
use crate::packs::repository::pack_compatibility::PackCompatibility;
use crate::shared_constants;
use crate::world::flag::feature_flag_set::FeatureFlagSet;

#[derive(Clone, Debug)]
pub struct Pack {
    pub location: PackLocationInfo,
    pub selection_config: PackSelectionConfig,
}
impl Pack {
    pub fn read_meta_and_create<T: PackResources>(
        location: PackLocationInfo,
        supplier: impl ResourcesSupplier<T>,
        pack_type: PackType,
        selection_config: PackSelectionConfig,
    ) -> Self {
        let pack_version = shared_constants::WORLD_VERSION.get_pack_version(pack_type);
        todo!();
    }

    pub fn is_required(&self) -> bool {
        self.selection_config.required
    }
}
impl PartialEq for Pack {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Position {
    Top,
    Bottom,
}
impl Position {
    // No opposite bool and no fn to get selection config as on the server, it's always the same values
    pub fn insert(&self, consumer: &mut Vec<Pack>, pack: Pack) -> usize {
        match self {
            Position::Top => {
                let mut insert_index = consumer.len() - 1;
                for (i, p) in consumer.iter().enumerate().rev() {
                    insert_index = i;
                    let selection_config = &p.selection_config;
                    if !selection_config.fixed_position
                        || selection_config.default_position != *self
                    {
                        break;
                    }
                }

                consumer.insert(insert_index, pack);
                insert_index
            }
            Position::Bottom => {
                let mut insert_index = 0;
                for (i, p) in consumer.iter().enumerate() {
                    insert_index = i;
                    let selection_config = &p.selection_config;
                    if !selection_config.fixed_position
                        || selection_config.default_position != *self
                    {
                        break;
                    }
                }

                consumer.insert(insert_index, pack);
                insert_index
            }
        }
    }
}

pub struct Metadata {
    description: MutableComponent,
    compatibility: PackCompatibility,
    requested_features: FeatureFlagSet,
    overlays: Vec<String>,
}
impl Metadata {
    pub fn read_pack_metadata<T: PackResources>(
        location: PackLocationInfo,
        supplier: impl ResourcesSupplier<T>,
        pack_version: u32,
    ) -> Self {
        let resources = supplier.open_primary(location);
        todo!();
    }
}

pub trait ResourcesSupplier<T: PackResources> {
    fn open_primary(&self, location: PackLocationInfo) -> &T;

    fn open_full(&self, location: PackLocationInfo, metadata: Metadata) -> &T;
}
