use crate::network::chat::component_contents::ComponentContents;
use crate::network::chat::contents::translatable_contents::TranslatableContents;
use crate::network::chat::style::Style;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct MutableComponent {
    contents: Arc<dyn ComponentContents>,
    siblings: Vec<MutableComponent>,
    style: Style,
}
impl MutableComponent {
    pub fn create(contents: Arc<dyn ComponentContents>) -> Self {
        Self {
            contents,
            siblings: Vec::new(),
            style: Style::default(),
        }
    }

    pub fn translatable(keys: &str) -> Self {
        Self::create(Arc::new(TranslatableContents::new(
            keys.to_string(),
            None,
            Vec::new(),
        )))
    }
}
