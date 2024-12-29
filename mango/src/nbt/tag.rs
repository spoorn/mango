use crate::nbt::compound_tag::CompoundTag;
use crate::nbt::end_tag::EndTag;
use serde::Serialize;
use strum::EnumTryAs;

/// We model Tags as an enum as it is unlikely we would need to customize this
///
/// Simple tags are represented as their primitive types, while complex tags have their own structs
#[derive(EnumTryAs, Serialize)]
pub enum Tag {
    EndTag(EndTag),
    ByteTag(u8),
    ShortTag(i16),
    IntTag(i32),
    LongTag(i64),
    FloatTag(f32),
    DoubleTag(f64),
    ByteArrayTag(Vec<u8>),
    StringTag(String),
    ListTag(Vec<Tag>),
    CompoundTag(CompoundTag),
    IntArrayTag(Vec<i32>),
    LongArrayTag(Vec<i64>),
}
impl Tag {
    pub fn get_as_int(&self) -> i32 {
        match self {
            Tag::ByteTag(e) => *e as i32,
            Tag::ShortTag(e) => *e as i32,
            Tag::IntTag(e) => *e,
            Tag::LongTag(e) => *e as i32,
            Tag::FloatTag(e) => *e as i32,
            Tag::DoubleTag(e) => *e as i32,
            _ => panic!("Invalid cast of numeric tag to i32"),
        }
    }

    pub fn get_as_long(&self) -> i64 {
        match self {
            Tag::ByteTag(e) => *e as i64,
            Tag::ShortTag(e) => *e as i64,
            Tag::IntTag(e) => *e as i64,
            Tag::LongTag(e) => *e,
            Tag::FloatTag(e) => *e as i64,
            Tag::DoubleTag(e) => *e as i64,
            _ => panic!("Invalid cast of numeric tag to i32"),
        }
    }

    pub fn get_as_string(&self) -> String {
        match self {
            Tag::StringTag(e) => e.clone(),
            _ => panic!("Invalid cast of string tag to String"),
        }
    }
}
