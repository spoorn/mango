use crate::core::block_pos::BlockPos;
use crate::core::direction::Direction;
use crate::core::registries::built_in_registries;
use crate::resources::resource_key::ResourceKey;
use crate::util::mth;
use crate::world::level::block::block::Block;
use crate::world::level::block::sound_type::SoundType;
use crate::world::level::block::state::block_behavior::properties_builder::{
    IsUnset, SetCanOcclude, SetDestroyTime, SetExplosionResistance, SetHasCollision,
    SetIsRandomlyTicking, SetOffsetFunction, State,
};
use crate::world::level::block::support_type::SupportType;
use crate::world::level::block::{block, blocks};
use crate::world::level::block_getter::BlockGetter;
use crate::world::level::material::map_color;
use crate::world::level::material::map_color::MapColor;
use crate::world::level::material::push_reaction::PushReaction;
use crate::world::phys::shapes::shapes;
use crate::world::phys::shapes::voxel_shape::VoxelShapeTrait;
use crate::world::phys::vec3::Vec3;
use bon::Builder;
use std::borrow::Borrow;

/// Function that checks a boolean property of a block
type BlockCheckFn = fn(&BlockState, &dyn BlockGetter, BlockPos) -> bool;

pub trait BlockBehaviour {
    fn get_block(&self) -> &Block;
    fn has_collision(&self) -> bool {
        self.get_block().has_collision()
    }
    fn properties(&self) -> &Properties {
        self.get_block().properties()
    }

    fn get_max_horizontal_offset(&self) -> f32 {
        0.25
    }

    // TODO: vanilla has a CollisionContext here but it's only used for LightBlock
    fn get_shape(
        &self,
        _block_state: &BlockState,
        _block_getter: &dyn BlockGetter,
        _block_pos: BlockPos,
    ) -> Box<dyn VoxelShapeTrait> {
        shapes::block()
    }

    fn get_collision_shape(
        &self,
        block_state: &BlockState,
        block_getter: &dyn BlockGetter,
        block_pos: BlockPos,
    ) -> Box<dyn VoxelShapeTrait> {
        if self.has_collision() {
            block_state.get_shape(block_getter, block_pos)
        } else {
            shapes::empty()
        }
    }

    fn is_collision_shape_full_block(
        &self,
        block_state: &BlockState,
        block_getter: &dyn BlockGetter,
        block_pos: BlockPos,
    ) -> bool {
        block::is_shape_full_block(
            block_state
                .get_collision_shape(block_getter, block_pos)
                .borrow(),
        )
    }
}

#[derive(Builder, Debug)]
#[builder(state_mod(vis = "pub"))]
pub struct Properties {
    pub id: Option<ResourceKey>,
    #[builder(default = |_| map_color::NONE)]
    pub map_color: fn(BlockState) -> MapColor,
    #[builder(default = true)]
    pub has_collision: bool,
    pub sound_type: SoundType,
    /// `light_emission` in vanilla
    #[builder(default = |_| 0)]
    pub light_level: fn(&BlockState) -> u8,
    #[builder(default = true)]
    pub can_occlude: bool,
    #[builder(default = false)]
    pub ignited_by_lava: bool,
    pub destroy_time: f32,
    #[builder(default = false)]
    pub requires_correct_tool_for_drops: bool,
    #[builder(default = false)]
    pub is_randomly_ticking: bool,
    #[builder(with = |r: f32| f32::max(0.0, r))]
    pub explosion_resistance: f32,
    #[builder(default = false)]
    pub force_solid_on: bool,
    #[builder(default)]
    pub push_reaction: PushReaction,
    #[builder(default = false)]
    pub replaceable: bool,
    #[builder(default = |block_state, block_getter, block_pos, _entity_type| block_state.is_face_sturdy(block_getter, block_pos, Direction::Up) && block_state.light_emission < 14)]
    pub is_valid_spawn: fn(&BlockState, &dyn BlockGetter, BlockPos, usize) -> bool,
    #[builder(default = |block_state, block_getter, block_pos| block_state.is_collision_shape_full_block(block_getter, block_pos))]
    pub is_redstone_conductor: BlockCheckFn,
    #[builder(default = |block_state, block_getter, block_pos| block_state.blocks_motion() && block_state.is_collision_shape_full_block(block_getter, block_pos))]
    pub is_suffocating: BlockCheckFn,
    /// Defaults to same as is_suffocating
    pub is_view_blocking: Option<BlockCheckFn>,
    pub offset_function: Option<fn(BlockState, BlockPos) -> Vec3>,
}

impl<S: State> PropertiesBuilder<S> {
    pub fn no_occlusion(self) -> PropertiesBuilder<SetCanOcclude<S>>
    where
        S::CanOcclude: IsUnset,
    {
        self.can_occlude(false)
    }

    pub fn no_collision(self) -> PropertiesBuilder<SetCanOcclude<SetHasCollision<S>>>
    where
        S::HasCollision: IsUnset,
        S::CanOcclude: IsUnset,
    {
        self.has_collision(false).can_occlude(false)
    }

    pub fn instabreak(self) -> PropertiesBuilder<SetExplosionResistance<SetDestroyTime<S>>>
    where
        S::ExplosionResistance: IsUnset,
        S::DestroyTime: IsUnset,
    {
        self.strength(0.0)
    }

    pub fn strength(
        self,
        strength: f32,
    ) -> PropertiesBuilder<SetExplosionResistance<SetDestroyTime<S>>>
    where
        S::ExplosionResistance: IsUnset,
        S::DestroyTime: IsUnset,
    {
        self.strength_with_res(strength, strength)
    }

    pub fn strength_with_res(
        self,
        destroy_time: f32,
        explosion_resistance: f32,
    ) -> PropertiesBuilder<SetExplosionResistance<SetDestroyTime<S>>>
    where
        S::ExplosionResistance: IsUnset,
        S::DestroyTime: IsUnset,
    {
        self.destroy_time(destroy_time)
            .explosion_resistance(explosion_resistance)
    }

    pub fn random_ticks(self) -> PropertiesBuilder<SetIsRandomlyTicking<S>>
    where
        S::IsRandomlyTicking: IsUnset,
    {
        self.is_randomly_ticking(true)
    }

    pub fn offset_type(self, offset_type: OffsetType) -> PropertiesBuilder<SetOffsetFunction<S>>
    where
        S::OffsetFunction: IsUnset,
    {
        match offset_type {
            OffsetType::None => self.maybe_offset_function(None),
            OffsetType::XZ => self.offset_function(|block_state, block_pos| {
                let block =
                    built_in_registries::get_block(block_state.block).expect("Block not found");
                let seed = mth::get_seed(block_pos.x, 0, block_pos.z);
                let max_y_offset = block.get_max_horizontal_offset() as f64;
                let x_offset =
                    (((seed & 15) as f64 / 15.0 - 0.5) * 0.5).clamp(-max_y_offset, max_y_offset);
                let z_offset = (((seed >> 8 & 15) as f64 / 15.0 - 0.5) * 0.5)
                    .clamp(-max_y_offset, max_y_offset);
                Vec3::new(x_offset, 0.0, z_offset)
            }),
            OffsetType::XYZ => self.offset_function(|block_state, block_pos| {
                let block =
                    built_in_registries::get_block(block_state.block).expect("Block not found");
                let seed = mth::get_seed(block_pos.x, 0, block_pos.z);
                let max_y_offset = block.get_max_horizontal_offset() as f64;
                let y_offset = ((seed >> 4 & 15) as f64 / 15.0 - 1.0) * max_y_offset;
                let x_offset =
                    (((seed & 15) as f64 / 15.0 - 0.5) * 0.5).clamp(-max_y_offset, max_y_offset);
                let z_offset = (((seed >> 8 & 15) as f64 / 15.0 - 0.5) * 0.5)
                    .clamp(-max_y_offset, max_y_offset);
                Vec3::new(x_offset, y_offset, z_offset)
            }),
        }
    }
}

pub struct BlockState {
    pub block: usize,
    pub light_emission: u8,
    legacy_solid: bool,
}
impl BlockState {
    pub fn new(block: usize) -> Self {
        let block_ref = built_in_registries::get_block(block).expect("Block not found");
        let mut res = Self {
            block,
            light_emission: 0,
            legacy_solid: false,
        };
        // Late bind property fields based on the instantiated BlockState
        res.light_emission = (block_ref.properties().light_level)(&res);
        res
    }

    pub fn blocks_motion(&self) -> bool {
        self.block != blocks::COBWEB.get().unwrap().id
            && self.block != blocks::BAMBOO_SAPLING.get().unwrap().id
            && self.is_solid()
    }

    pub fn is_solid(&self) -> bool {
        self.legacy_solid
    }

    pub fn get_shape(
        &self,
        block_getter: &dyn BlockGetter,
        block_pos: BlockPos,
    ) -> Box<dyn VoxelShapeTrait> {
        built_in_registries::get_block(self.block)
            .expect("Block not found")
            .get_shape(self, block_getter, block_pos)
    }

    pub fn get_collision_shape(
        &self,
        block_getter: &dyn BlockGetter,
        block_pos: BlockPos,
    ) -> Box<dyn VoxelShapeTrait> {
        built_in_registries::get_block(self.block)
            .expect("Block not found")
            .get_collision_shape(self, block_getter, block_pos)
    }

    // TODO: cache this
    pub fn is_collision_shape_full_block(
        &self,
        block_getter: &dyn BlockGetter,
        block_pos: BlockPos,
    ) -> bool {
        built_in_registries::get_block(self.block)
            .expect("Block not found")
            .is_collision_shape_full_block(self, block_getter, block_pos)
    }

    pub fn is_face_sturdy(
        &self,
        block_getter: &dyn BlockGetter,
        block_pos: BlockPos,
        direction: Direction,
    ) -> bool {
        self.is_face_sturdy_for_support_type(block_getter, block_pos, direction, SupportType::Full)
    }

    // TODO: cache this
    pub fn is_face_sturdy_for_support_type(
        &self,
        block_getter: &dyn BlockGetter,
        block_pos: BlockPos,
        direction: Direction,
        support_type: SupportType,
    ) -> bool {
        support_type.is_supporting(self, block_getter, block_pos, direction)
    }
}

pub enum OffsetType {
    None,
    XZ,
    XYZ,
}
