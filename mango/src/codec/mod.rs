use anyhow::Result;

// TODO: Lifecycle is used in Codecs
pub trait Codec {
    // TODO: use impl Trait: https://github.com/rust-lang/rust/issues/63063 for all implementations
    type Data;

    fn decode(data: Self::Data) -> Result<Self>
    where
        Self: Sized;
}
