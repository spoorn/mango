use std::fmt::Debug;

pub trait BlockTrait: Send + Sync + Debug {}

#[derive(Debug)]
pub struct Block {}

impl BlockTrait for Block {}
