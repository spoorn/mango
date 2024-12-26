use std::fmt::Debug;
use std::sync::Arc;

pub trait BlockTrait: Send + Sync + Debug {}

#[derive(Debug)]
pub struct Block {}

impl BlockTrait for Block {}

impl<T: ?Sized + BlockTrait> BlockTrait for Arc<T> {}
