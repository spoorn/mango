#[derive(Default)]
pub enum PushReaction {
    #[default]
    Normal,
    Destroy,
    Block,
    Ignore,
    PushOnly,
}
