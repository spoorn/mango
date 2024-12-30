use crate::nbt::nbt_accounter::NbtAccounter;
use crate::nbt::tag::Tag;
use crate::nbt::tag_type::TagType;
use crate::nbt::DataInput;
use anyhow::Result;
use serde::Serialize;
use std::borrow::Borrow;
use std::ops::Deref;
use std::sync::Arc;
use tokio::io::AsyncReadExt;

#[derive(Clone, Default, Serialize)]
pub struct ListTag {
    pub tags: Arc<Vec<Tag>>,
}
impl ListTag {
    pub fn new(tags: Vec<Tag>) -> Self {
        Self {
            tags: Arc::new(tags),
        }
    }
}
impl Deref for ListTag {
    type Target = Vec<Tag>;

    fn deref(&self) -> &Self::Target {
        &self.tags
    }
}

pub async fn load_list(
    reader: &mut DataInput,
    nbt_accounter: impl Borrow<NbtAccounter>,
) -> Result<ListTag> {
    let nbt_accounter = nbt_accounter.borrow();
    nbt_accounter.account_bytes(37);

    let tag_type = reader.read_u8().await?;
    let len = reader.read_i32().await?;

    if tag_type == 0 && len > 0 {
        panic!("Missing type on ListTag");
    }

    nbt_accounter.account_bytes(4 * len as u64);
    let mut list = Vec::with_capacity(len as usize);

    for _ in 0..len {
        list.push(Box::pin(TagType::get_type(tag_type).load(reader, nbt_accounter)).await?);
    }

    Ok(ListTag::new(list))
}
