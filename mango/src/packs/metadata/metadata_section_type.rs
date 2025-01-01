// https://stackoverflow.com/questions/75654498/clone-custom-structs-of-concrete-type-as-trait-objects
// #![feature(unsize, coerce_unsized)]
// use std::marker::Unsize;
// use std::ops::CoerceUnsized;

use crate::packs::metadata::pack::MetadataSection;
use anyhow::Result;
use serde_json::Value;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct MetadataSectionType {
    pub name: &'static str,
    pub codec: fn(Value) -> Result<Box<dyn MetadataSection>>,
}
impl MetadataSectionType {
    pub const fn new(
        name: &'static str,
        codec: fn(Value) -> Result<Box<dyn MetadataSection>>,
    ) -> Self {
        Self { name, codec }
    }
}
impl Hash for MetadataSectionType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
impl PartialEq for MetadataSectionType {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for MetadataSectionType {}
// impl<U: ?Sized, T: Unsize<U>> CoerceUnsized<MetadataSectionType<U>> for MetadataSectionType<T> {}
