use serde::Serialize;

pub const INSTANCE: EndTag = EndTag {};

#[derive(Serialize)]
pub struct EndTag {}
