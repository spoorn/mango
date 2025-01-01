use std::fmt::Debug;

pub mod feature_flags_metadata_section;
pub mod pack_metadata_section;

// Empty trait to group metadata section implementations
pub trait MetadataSection: Debug {
    /// We should probably use an enum instead of Any for MetadataSections as there are only 4
    /// implementors in vanilla, but I had some neat tricks to get all of this working and there
    /// should not be a significant performance impact so keeping it for the legacy.
    fn as_any(&self) -> &dyn std::any::Any;
}
