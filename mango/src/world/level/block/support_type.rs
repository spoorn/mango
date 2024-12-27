use crate::core::block_pos::BlockPos;
use crate::core::direction::Direction;
use crate::world::level::block::state::block_behavior::BlockState;
use crate::world::level::block_getter::BlockGetter;

pub enum SupportType {
    Full,
    Center,
    Rigid,
}
impl SupportType {
    // TODO: implement this
    pub fn is_supporting(
        &self,
        block_state: &BlockState,
        block_getter: &dyn BlockGetter,
        block_pos: BlockPos,
        direction: Direction,
    ) -> bool {
        match self {
            SupportType::Full => false,
            SupportType::Center => false,
            SupportType::Rigid => false,
        }
    }
}
