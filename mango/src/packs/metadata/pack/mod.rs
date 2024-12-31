use std::fmt::Debug;

pub mod feature_flags_metadata_section;
pub mod pack_metadata_section;

// Empty trait to group metadata section implementations
pub trait MetadataSection: Debug {}
