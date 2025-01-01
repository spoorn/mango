use crate::network::chat::component_contents::ComponentContents;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct PlainTextContents {
    text: String,
}
impl PlainTextContents {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}
impl Display for PlainTextContents {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.text.is_empty() {
            write!(f, "empty")
        } else {
            write!(f, "literal{{{}}}", self.text)
        }
    }
}
impl ComponentContents for PlainTextContents {}
