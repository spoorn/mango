use crate::network::chat::component_contents::ComponentContents;
use crate::network::chat::contents::translatable_contents::TranslatableContents;
use crate::network::chat::style::Style;

#[derive(Debug)]
pub struct MutableComponent {
    contents: Box<dyn ComponentContents>,
    siblings: Vec<MutableComponent>,
    style: Style,
}
impl MutableComponent {
    pub fn create(contents: Box<dyn ComponentContents>) -> Self {
        Self {
            contents,
            siblings: Vec::new(),
            style: Style::default(),
        }
    }

    pub fn translatable(keys: &str) -> Self {
        Self::create(Box::new(TranslatableContents::new(
            keys.to_string(),
            None,
            Vec::new(),
        )))
    }
}
