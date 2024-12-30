use async_compression::tokio::bufread::GzipDecoder;
use tokio::fs::File;
use tokio::io::BufReader;

pub mod compound_tag;
mod end_tag;
pub mod list_tag;
pub mod nbt_accounter;
pub mod nbt_io;
pub mod nbt_ops;
pub mod nbt_utils;
mod string_tag;
pub mod tag;
pub mod tag_type;

/// Helper type for reading compressed NBT data
type DataInput = GzipDecoder<BufReader<File>>;
