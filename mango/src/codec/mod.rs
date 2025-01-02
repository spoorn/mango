use anyhow::Result;

// TODO: Lifecycle is used in Codecs
pub trait Codec<T> {
    fn decode(data: T) -> Result<Self>
    where
        Self: Sized;

    fn decode_boxed(data: T) -> Result<Box<Self>>
    where
        Self: Sized,
    {
        Self::decode(data).map(Box::new)
    }
}
