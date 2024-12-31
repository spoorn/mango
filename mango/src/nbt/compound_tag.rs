use crate::nbt::list_tag::ListTag;
use crate::nbt::nbt_accounter::NbtAccounter;
use crate::nbt::tag::Tag;
use crate::nbt::tag_type::TagType;
use crate::nbt::DataInput;
use anyhow::Result;
use dashmap::DashMap;
use serde::Serialize;
use std::borrow::Borrow;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tracing::error;

/// CompoundTag that is thread-safe and cloneable
#[derive(Default, Clone, Serialize)]
pub struct CompoundTag {
    tags: Arc<DashMap<String, Tag>>,
}
impl CompoundTag {
    pub async fn load_compound(
        reader: &mut DataInput,
        nbt_accounter: impl Borrow<NbtAccounter>,
    ) -> Result<CompoundTag> {
        let accounter = nbt_accounter.borrow();
        accounter.account_bytes(48);
        let map = DashMap::new();

        let mut tag_type = 1;
        while tag_type != 0 {
            tag_type = reader.read_u8().await?;
            if tag_type == 0 {
                break;
            }
            let name = Self::read_string(reader, accounter).await?;
            let tag =
                Self::read_named_tag_data(TagType::get_type(tag_type), &name, reader, accounter)
                    .await?;
            if map.insert(name, tag).is_none() {
                accounter.account_bytes(36);
            }
        }

        Ok(Self {
            tags: Arc::new(map),
        })
    }

    /// Reads a modified UTF-8 string based on
    pub async fn read_string(
        reader: &mut DataInput,
        nbt_accounter: &NbtAccounter,
    ) -> Result<String> {
        let len = reader.read_u16().await?;
        let mut buf = vec![0u8; len as usize];
        reader.read_exact(&mut buf).await?;
        let res = cesu8::from_java_cesu8(&buf)?.to_string();
        nbt_accounter.account_bytes(28 + 2 * res.len() as u64);
        Ok(res)
    }

    async fn read_named_tag_data(
        tag_type: TagType,
        name: &String,
        reader: &mut DataInput,
        nbt_accounter: impl Borrow<NbtAccounter>,
    ) -> Result<Tag> {
        Box::pin(tag_type.load(reader, nbt_accounter))
            .await
            .inspect_err(|e| {
                // TODO: crash report
                error!(
                    "Failed to load tag {} of type {:?} in CompoundTag: {}",
                    name, tag_type, e
                );
            })
    }

    pub fn get_tag_type(&self, tag: impl AsRef<str>) -> TagType {
        match self.tags.get(tag.as_ref()) {
            Some(tag) => TagType::from(tag.value()),
            // vanilla defaults to end tag
            None => TagType::EndTag,
        }
    }

    pub fn contains(&self, tag: impl AsRef<str>, tag_type: TagType) -> bool {
        let extracted_type = self.get_tag_type(tag);
        extracted_type == tag_type || (tag_type == TagType::Numeric && extracted_type.is_numeric())
    }

    pub fn get_int_or_default(&self, tag: impl AsRef<str>, default: i32) -> i32 {
        if self.contains(&tag, TagType::Numeric) {
            return self.tags.get(tag.as_ref()).unwrap().value().get_as_int();
        }
        default
    }

    pub fn get_int(&self, tag: impl AsRef<str>) -> i32 {
        self.get_int_or_default(tag, 0)
    }

    pub fn get_long_or_default(&self, tag: impl AsRef<str>, default: i64) -> i64 {
        if self.contains(&tag, TagType::Numeric) {
            return self.tags.get(tag.as_ref()).unwrap().value().get_as_long();
        }
        default
    }

    pub fn get_long(&self, tag: impl AsRef<str>) -> i64 {
        self.get_long_or_default(tag, 0)
    }

    pub fn get_string_or_default(&self, tag: impl AsRef<str>, default: String) -> String {
        if self.contains(&tag, TagType::StringTag) {
            return self.tags.get(tag.as_ref()).unwrap().value().get_as_string();
        }
        default
    }

    pub fn get_string(&self, tag: impl AsRef<str>) -> String {
        self.get_string_or_default(tag, String::new())
    }

    pub fn try_get_string(&self, tag: impl AsRef<str>) -> Option<String> {
        self.tags
            .get(tag.as_ref())?
            .try_as_string_tag_ref()
            .cloned()
    }

    pub fn get_bool_or_default(&self, tag: impl AsRef<str>, default: bool) -> bool {
        if self.contains(&tag, TagType::ByteTag) {
            return self.tags.get(tag.as_ref()).unwrap().value().get_as_int() != 0;
        }
        default
    }

    pub fn get_bool(&self, tag: impl AsRef<str>) -> bool {
        self.get_bool_or_default(tag, false)
    }

    pub fn get_list(&self, tag: impl AsRef<str>) -> ListTag {
        if self.contains(&tag, TagType::ListTag) {
            return self
                .tags
                .get(tag.as_ref())
                .unwrap()
                .value()
                .try_as_list_tag_ref()
                .unwrap()
                .clone();
        }
        ListTag::default()
    }

    pub fn try_get_list(&self, tag: impl AsRef<str>) -> Option<ListTag> {
        Some(self.tags.get(tag.as_ref())?.try_as_list_tag_ref()?.clone())
    }

    pub fn get_compound<'a>(&self, tag: impl AsRef<str>) -> CompoundTag {
        // TODO: crash report
        if self.contains(&tag, TagType::CompoundTag) {
            return self
                .tags
                .get(tag.as_ref())
                .unwrap()
                .try_as_compound_tag_ref()
                .unwrap()
                .clone();
        }
        Self::default()
    }

    pub fn try_get_compound<'a>(&self, tag: impl AsRef<str>) -> Option<CompoundTag> {
        Some(
            self.tags
                .get(tag.as_ref())?
                .try_as_compound_tag_ref()?
                .clone(),
        )
    }
}
