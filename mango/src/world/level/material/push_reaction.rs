use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub enum PushReaction {
    #[default]
    Normal,
    Destroy,
    Block,
    Ignore,
    PushOnly,
}
