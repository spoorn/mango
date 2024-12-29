use crate::nbt::compound_tag::CompoundTag;
use crate::nbt::nbt_accounter::NbtAccounter;
use crate::nbt::tag::Tag;
use crate::nbt::tag_type::TagType;
use crate::nbt::{end_tag, string_tag, DataInput};
use anyhow::{anyhow, Result};
use async_compression::tokio::bufread::GzipDecoder;
use std::borrow::Borrow;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::BufReader;

pub async fn read_compressed(
    path: PathBuf,
    nbt_accounter: impl Borrow<NbtAccounter>,
) -> Result<CompoundTag> {
    let reader = GzipDecoder::new(BufReader::new(File::open(&path).await?));
    read(reader, nbt_accounter).await
}

async fn read(
    mut reader: DataInput,
    nbt_accounter: impl Borrow<NbtAccounter>,
) -> Result<CompoundTag> {
    match read_unnamed_tag(&mut reader, nbt_accounter).await? {
        Tag::CompoundTag(tag) => Ok(tag),
        _ => Err(anyhow!("Root tag must be a named compound tag")),
    }
}

async fn read_unnamed_tag(
    reader: &mut DataInput,
    nbt_accounter: impl Borrow<NbtAccounter>,
) -> Result<Tag> {
    let tag_type = reader.read_u8().await?;
    if tag_type == 0 {
        Ok(Tag::EndTag(end_tag::INSTANCE))
    } else {
        string_tag::skip_string(reader).await?;
        read_tag_safe(reader, nbt_accounter, tag_type).await
    }
}

async fn read_tag_safe(
    reader: &mut DataInput,
    nbt_accounter: impl Borrow<NbtAccounter>,
    tag_type: u8,
) -> Result<Tag> {
    // TODO: crash report
    Ok(TagType::get_type(tag_type)
        .load(reader, nbt_accounter)
        .await?)
}
