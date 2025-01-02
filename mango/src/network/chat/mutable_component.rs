use crate::network::chat::component_contents::ComponentContents;
use crate::network::chat::contents::plain_text_contents::PlainTextContents;
use crate::network::chat::contents::translatable_contents::TranslatableContents;
use crate::network::chat::style::Style;
use serde::{Deserialize, Deserializer};
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

    pub fn literal(id: &str) -> Self {
        Self::create(Arc::new(PlainTextContents::new(id.to_string())))
    }

    pub fn translatable(keys: &str) -> Self {
        Self::create(Arc::new(TranslatableContents::new(
            keys.to_string(),
            None,
            Vec::new(),
        )))
    }

    /// Deserializes the "description" field in pack.mcmeta
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let json = serde_json::Value::deserialize(deserializer)?;
        if let Some(translate) = json.get("translate") {
            Ok(Self::translatable(translate.as_str().expect(
                "pack.mcmeta translate was not a valid JSON string",
            )))
        } else if let Some(literal) = json.get("text") {
            Ok(Self::literal(
                literal
                    .as_str()
                    .expect("pack.mcmeta text was not a valid JSON string"),
            ))
        } else {
            Err(serde::de::Error::custom(
                "pack.mcmeta description field is not valid or supported",
            ))
        }
    }
}
