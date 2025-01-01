use crate::packs::repository::pack_repository::PackRepository;
use crate::world::flag::feature_flag_set::FeatureFlagSet;
use crate::world::flag::feature_flags;
use crate::world::level::world_data_configuration::WorldDataConfiguration;

pub struct MinecraftServer {}

fn configure_pack_repository(
    mut pack_repo: PackRepository,
    world_data_configuration: WorldDataConfiguration,
    init_mode: bool,
    safe_mode: bool,
) {
    let start_flags = if init_mode {
        &FeatureFlagSet::empty()
    } else {
        &world_data_configuration.enabled_features
    };
    let all_flags = if init_mode {
        &feature_flags::FEATURE_FLAGS.registry.all_flags
    } else {
        &world_data_configuration.enabled_features
    };

    pack_repo.reload();
    todo!();
}
