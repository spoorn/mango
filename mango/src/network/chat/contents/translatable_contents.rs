use crate::network::chat::component_contents::ComponentContents;
use std::any::Any;

#[derive(Debug)]
pub struct TranslatableContents {
    key: String,
    fallback: Option<String>,
    // TODO: Can we avoid Any here?
    args: Vec<Box<dyn Any>>,
}
impl TranslatableContents {
    pub fn new(key: String, fallback: Option<String>, args: Vec<Box<dyn Any>>) -> Self {
        Self {
            key,
            fallback,
            args,
        }
    }
}
impl ComponentContents for TranslatableContents {}
