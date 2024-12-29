use crate::nbt::compound_tag::CompoundTag;
use crate::nbt::nbt_accounter::NbtAccounter;
use crate::nbt::tag::Tag;
use crate::nbt::{end_tag, list_tag, DataInput};
use anyhow::anyhow;
use std::borrow::Borrow;
use strum::{EnumCount, FromRepr};
use tokio::io::AsyncReadExt;

/// Tag types. Only used internally here for split processing for [crate::nbt::tag::Tag]
#[derive(EnumCount, FromRepr, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum TagType {
    EndTag,
    ByteTag,
    ShortTag,
    IntTag,
    LongTag,
    FloatTag,
    DoubleTag,
    ByteArrayTag,
    StringTag,
    ListTag,
    CompoundTag,
    IntArrayTag,
    LongArrayTag,
    Invalid(u8),
    /// Special case
    Numeric = 99,
}
impl TagType {
    pub fn get_type(tag_type: u8) -> TagType {
        // Has to be in one of the variants before Invalid
        if tag_type > (TagType::COUNT - 3) as u8 {
            TagType::Invalid(tag_type)
        } else {
            TagType::from_repr(tag_type).unwrap()
        }
    }

    /// Check if the type is a numeric type (not the special case [Self::Numeric] variant)
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            Self::ByteTag
                | Self::ShortTag
                | Self::IntTag
                | Self::LongTag
                | Self::FloatTag
                | Self::DoubleTag
        )
    }

    pub async fn load(
        &self,
        reader: &mut DataInput,
        nbt_accounter: impl Borrow<NbtAccounter>,
    ) -> anyhow::Result<Tag> {
        match self {
            TagType::EndTag => {
                nbt_accounter.borrow().account_bytes(8);
                Ok(Tag::EndTag(end_tag::INSTANCE))
            }
            TagType::ByteTag => {
                nbt_accounter.borrow().account_bytes(9);
                Ok(Tag::ByteTag(reader.read_u8().await?))
            }
            TagType::ShortTag => {
                nbt_accounter.borrow().account_bytes(10);
                Ok(Tag::ShortTag(reader.read_i16().await?))
            }
            TagType::IntTag => {
                nbt_accounter.borrow().account_bytes(12);
                Ok(Tag::IntTag(reader.read_i32().await?))
            }
            TagType::LongTag => {
                nbt_accounter.borrow().account_bytes(16);
                Ok(Tag::LongTag(reader.read_i64().await?))
            }
            TagType::FloatTag => {
                nbt_accounter.borrow().account_bytes(12);
                Ok(Tag::FloatTag(reader.read_f32().await?))
            }
            TagType::DoubleTag => {
                nbt_accounter.borrow().account_bytes(16);
                Ok(Tag::DoubleTag(reader.read_f64().await?))
            }
            TagType::ByteArrayTag => {
                let nbt_accounter = nbt_accounter.borrow();
                nbt_accounter.account_bytes(24);
                let len = reader.read_i32().await?;
                nbt_accounter.account_bytes(len as u64);
                let mut buf = vec![0u8; len as usize];
                reader.read_exact(&mut buf).await?;
                Ok(Tag::ByteArrayTag(buf))
            }
            TagType::StringTag => {
                let nbt_accounter = nbt_accounter.borrow();
                nbt_accounter.account_bytes(36);
                let len = reader.read_u16().await?;
                let mut buf = vec![0u8; len as usize];
                reader.read_exact(&mut buf).await?;
                let res = cesu8::from_java_cesu8(&buf)?.to_string();
                nbt_accounter.account_bytes(28 + 2 * res.len() as u64);
                Ok(Tag::StringTag(res))
            }
            TagType::ListTag => list_tag::load_list(reader, nbt_accounter)
                .await
                .map(Tag::ListTag),
            TagType::CompoundTag => {
                let nbt_accounter = nbt_accounter.borrow();
                nbt_accounter.push_depth();
                match CompoundTag::load_compound(reader, nbt_accounter).await {
                    Ok(res) => {
                        nbt_accounter.pop_depth();
                        Ok(Tag::CompoundTag(res))
                    }
                    Err(e) => {
                        nbt_accounter.pop_depth();
                        Err(e)
                    }
                }
            }
            TagType::IntArrayTag => {
                let nbt_accounter = nbt_accounter.borrow();
                nbt_accounter.account_bytes(24);
                let len = reader.read_i32().await?;
                nbt_accounter.account_bytes(4 * len as u64);
                let mut buf = vec![0i32; len as usize];
                for i in 0..len {
                    buf[i as usize] = reader.read_i32().await?;
                }
                Ok(Tag::IntArrayTag(buf))
            }
            TagType::LongArrayTag => {
                let nbt_accounter = nbt_accounter.borrow();
                nbt_accounter.account_bytes(24);
                let len = reader.read_i32().await?;
                nbt_accounter.account_bytes(8 * len as u64);
                let mut buf = vec![0i64; len as usize];
                for i in 0..len {
                    buf[i as usize] = reader.read_i64().await?;
                }
                Ok(Tag::LongArrayTag(buf))
            }
            TagType::Invalid(tag_id) => Err(anyhow!("Invalid tag id: {}", tag_id)),
            TagType::Numeric => {
                todo!();
            }
        }
    }
}
impl From<&Tag> for TagType {
    fn from(value: &Tag) -> Self {
        match value {
            Tag::EndTag(_) => Self::EndTag,
            Tag::ByteTag(_) => Self::ByteTag,
            Tag::ShortTag(_) => Self::ShortTag,
            Tag::IntTag(_) => Self::IntTag,
            Tag::LongTag(_) => Self::LongTag,
            Tag::FloatTag(_) => Self::FloatTag,
            Tag::DoubleTag(_) => Self::DoubleTag,
            Tag::ByteArrayTag(_) => Self::ByteArrayTag,
            Tag::StringTag(_) => Self::StringTag,
            Tag::ListTag(_) => Self::ListTag,
            Tag::CompoundTag(_) => Self::CompoundTag,
            Tag::IntArrayTag(_) => Self::IntArrayTag,
            Tag::LongArrayTag(_) => Self::LongArrayTag,
        }
    }
}
