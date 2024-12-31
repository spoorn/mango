use std::fmt::Debug;

pub trait RepositorySource: Debug {
    fn load_packs(&self);
}
