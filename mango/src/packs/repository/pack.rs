use crate::network::chat::mutable_component::MutableComponent;
use crate::packs::metadata::pack::feature_flags_metadata_section::FeatureFlagsMetadataSection;
use crate::packs::metadata::pack::pack_metadata_section::PackMetadataSection;
use crate::packs::metadata::pack::{feature_flags_metadata_section, pack_metadata_section};
use crate::packs::pack_location_info::PackLocationInfo;
use crate::packs::pack_resources::PackResources;
use crate::packs::pack_selection_config::PackSelectionConfig;
use crate::packs::pack_type::PackType;
use crate::packs::repository::pack_compatibility::PackCompatibility;
use crate::shared_constants;
use crate::world::flag::feature_flag_set::FeatureFlagSet;
use std::fmt::{Debug, Display, Formatter};
use std::ops::RangeInclusive;
use std::rc::Rc;
use std::sync::Arc;
use tracing::warn;

#[derive(Clone, Debug)]
pub struct Pack {
    pub location: PackLocationInfo,
    pub resources: Rc<dyn ResourcesSupplier>,
    pub metadata: Metadata,
    pub selection_config: PackSelectionConfig,
}
impl Pack {
    pub fn read_meta_and_create(
        location: PackLocationInfo,
        // Make the compiler happy by making it static lifetime
        supplier: Rc<dyn ResourcesSupplier + 'static>,
        pack_type: PackType,
        selection_config: PackSelectionConfig,
    ) -> Option<Self> {
        let pack_version = shared_constants::WORLD_VERSION.get_pack_version(pack_type);
        let metadata = Metadata::read_pack_metadata(&location, &supplier, pack_version)?;
        Some(Self {
            location,
            resources: supplier,
            metadata,
            selection_config,
        })
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
impl Display for Pack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Pack {{\n\
        location: {:#?},\n\
        metadata: {:#?},\n\
        selection_config: {:#?}\n\
        }}",
            self.location, self.metadata, self.selection_config
        )
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

#[derive(Clone, Debug)]
pub struct Metadata {
    description: MutableComponent,
    compatibility: PackCompatibility,
    requested_features: FeatureFlagSet,
    overlays: Vec<String>,
}
impl Metadata {
    pub fn read_pack_metadata(
        location: &PackLocationInfo,
        supplier: &Rc<dyn ResourcesSupplier>,
        pack_version: u32,
    ) -> Option<Self> {
        let resources = supplier.open_primary(location);
        let pack_metadata_section = resources.get_metadata_section(pack_metadata_section::TYPE);
        if pack_metadata_section.is_none() {
            warn!("Missing metadata in pack {}", location.id);
            return None;
        }
        let pack_metadata_section = pack_metadata_section.unwrap();
        let pack_metadata_section = pack_metadata_section
            .as_any()
            .downcast_ref::<PackMetadataSection>()
            .expect("Invalid pack metadata section in pack");
        let feature_flags_metadata_section =
            resources.get_metadata_section(feature_flags_metadata_section::TYPE);
        let feature_flag_set = match feature_flags_metadata_section {
            None => FeatureFlagSet::empty(),
            Some(section) => {
                match section
                    .as_any()
                    .downcast_ref::<FeatureFlagsMetadataSection>()
                {
                    None => panic!("Invalid feature flags section in pack {}", location.id),
                    Some(feature_flags_section) => feature_flags_section.flags.clone(),
                }
            }
        };
        let range = Self::get_declared_pack_versions(&location.id, pack_metadata_section);
        let pack_compatibility = PackCompatibility::for_version(range, pack_version);
        // TODO: OverlayMetadataSection does not seem to be used in vanilla server for built in metadata
        Some(Self {
            description: pack_metadata_section.description.clone(),
            compatibility: pack_compatibility,
            requested_features: feature_flag_set,
            overlays: Vec::new(),
        })
    }

    fn get_declared_pack_versions(
        id: &String,
        pack_metadata_section: &PackMetadataSection,
    ) -> RangeInclusive<u32> {
        match &pack_metadata_section.supported_formats {
            None => 0..=pack_metadata_section.pack_format,
            Some(supported_formats) => {
                if supported_formats.contains(&pack_metadata_section.pack_format) {
                    supported_formats.clone()
                } else {
                    warn!("Pack {} declared support for versions {:?} but declared main format is {}, defaulting to {}",
                        id,
                        supported_formats,
                        pack_metadata_section.pack_format,
                        pack_metadata_section.pack_format);
                    0..=pack_metadata_section.pack_format
                }
            }
        }
    }
}

pub trait ResourcesSupplier: Debug {
    fn open_primary(&self, location: &PackLocationInfo) -> Arc<dyn PackResources>;

    /// All implementations of this do the same thing as open_primary because the Metadata has
    /// no overlays in the vanilla server
    fn open_full(&self, location: &PackLocationInfo, metadata: &Metadata)
        -> Arc<dyn PackResources>;
}
