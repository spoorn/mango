#[derive(Default, Debug, Eq, PartialEq, Clone)]
pub enum PackSource {
    #[default]
    Default,
    BuiltIn,
    Feature,
    World,
    Server,
}
