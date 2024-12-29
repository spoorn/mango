use crate::nbt::DataInput;
use anyhow::Result;
use std::io::SeekFrom;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

/// Skip some bytes
///
/// https://stackoverflow.com/questions/59826242/how-to-skip-n-bytes-with-read-without-allocation
pub async fn skip_string(reader: &mut DataInput) -> Result<()> {
    let skip_bytes = reader.read_u16().await?;
    reader
        .get_mut()
        .seek(SeekFrom::Current(skip_bytes as i64))
        .await?;
    Ok(())
}
