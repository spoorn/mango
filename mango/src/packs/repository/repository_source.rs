use crate::packs::repository::pack::Pack;
use std::fmt::Debug;

pub trait RepositorySource: Debug {
    fn load_packs(&self, consumer: &mut dyn FnMut(Pack) -> ());
}
