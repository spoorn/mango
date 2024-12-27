use crate::resources::resource_key::ResourceKey;
use crate::util;
use crate::world::item::item::properties_builder::{IsUnset, SetDescriptionId, State};
use bon::Builder;
use serde::Serialize;
use std::fmt::Debug;

const BLOCK_DESCRIPTION_ID: fn(ResourceKey) -> String =
    |key| util::make_description_id("block", key.location);
const ITEM_DESCRIPTION_ID: fn(ResourceKey) -> String =
    |key| util::make_description_id("item", key.location);

#[derive(Builder)]
#[builder(state_mod(vis = "pub"))]
pub struct Properties {
    pub id: Option<ResourceKey>,
    #[builder(default = ITEM_DESCRIPTION_ID)]
    description_id: fn(ResourceKey) -> String,
}
impl<S: State> PropertiesBuilder<S> {
    pub fn use_block_description_prefix(self) -> PropertiesBuilder<SetDescriptionId<S>>
    where
        S::DescriptionId: IsUnset,
    {
        self.description_id(BLOCK_DESCRIPTION_ID)
    }
}

#[typetag::serialize(tag = "type")]
pub trait ItemTrait: Send + Sync + Debug {
    // instead of instanceof
    fn is_block_item(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
pub struct Item {}
#[typetag::serialize]
impl ItemTrait for Item {}
impl Item {
    pub fn new(properties: Properties) -> Self {
        Self {}
    }
}
