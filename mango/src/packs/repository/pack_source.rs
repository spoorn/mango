#[derive(Default, Debug, Eq, PartialEq, Clone)]
pub enum PackSource {
    #[default]
    Default,
    BuiltIn,
    Feature,
    World,
    Server,
}
impl PackSource {
    pub fn should_add_automatically(&self) -> bool {
        match self {
            PackSource::Default => true,
            PackSource::BuiltIn => true,
            PackSource::Feature => false,
            PackSource::World => true,
            PackSource::Server => true,
        }
    }
}
