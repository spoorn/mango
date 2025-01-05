use crate::packs::repository::pack_repository::PackRepository;
use crate::world::flag::feature_flag_set::FeatureFlagSet;
use crate::world::flag::feature_flags;
use crate::world::level::data_pack_config::DataPackConfig;
use crate::world::level::world_data_configuration::WorldDataConfiguration;
use std::collections::BTreeSet;
use tracing::{info, warn};

pub struct MinecraftServer {}

pub fn configure_pack_repository(
    pack_repo: &mut PackRepository,
    world_data_configuration: &WorldDataConfiguration,
    init_mode: bool,
    safe_mode: bool,
) -> WorldDataConfiguration {
    let start_flags = if init_mode {
        FeatureFlagSet::empty()
    } else {
        world_data_configuration.enabled_features.clone()
    };
    let all_flags = if init_mode {
        &feature_flags::FEATURE_FLAGS.registry.all_flags
    } else {
        &world_data_configuration.enabled_features
    };

    pack_repo.reload();

    if safe_mode {
        return configure_repository_with_selection(
            pack_repo,
            vec!["vanilla".to_string()],
            start_flags,
            false,
        );
    }

    let mut enabled_packs = BTreeSet::new();
    world_data_configuration
        .datapacks
        .enabled
        .iter()
        .for_each(|id| {
            if pack_repo.is_available(id) {
                enabled_packs.insert(id.clone());
            } else {
                warn!("Missing data pack {}", id);
            }
        });

    pack_repo.available.values().for_each(|pack| {
        let id = &pack.location.id;
        if !world_data_configuration.datapacks.disabled.contains(id) {
            let requested_features = &pack.metadata.requested_features;
            let was_enabled = enabled_packs.contains(id);
            if !was_enabled && pack.location.source.should_add_automatically() {
                if requested_features.is_subset_of(all_flags) {
                    info!("Found new data pack {}, loading it automatically", id);
                    enabled_packs.insert(id.clone());
                } else {
                    warn!(
                        "Found new data pack {}, but can't load it due to missing features {}",
                        id,
                        feature_flags::print_missing_flags(all_flags, requested_features)
                    )
                }
            }

            if was_enabled && !requested_features.is_subset_of(all_flags) {
                warn!("Pack {} requires features {} that are not enabled for this world, disabling pack.", id, feature_flags::print_missing_flags(all_flags, requested_features));
                enabled_packs.remove(id);
            }
        }
    });

    if enabled_packs.is_empty() {
        info!("No datapacks selected, forcing vanilla");
        enabled_packs.insert("vanilla".to_string());
    }

    configure_repository_with_selection(
        pack_repo,
        Vec::from_iter(enabled_packs.into_iter()),
        start_flags,
        true,
    )
}

fn configure_repository_with_selection(
    pack_repo: &mut PackRepository,
    selected_packs: Vec<String>,
    flags: FeatureFlagSet,
    include_disabled: bool,
) -> WorldDataConfiguration {
    pack_repo.set_selected(&selected_packs.iter().collect::<Vec<&String>>());
    enable_forced_feature_packs(pack_repo, &flags);
    let data_pack_config = get_selected_packs(&pack_repo, include_disabled);
    let flags = pack_repo.get_requested_feature_flags().join(flags);
    WorldDataConfiguration::new(data_pack_config, flags)
}

fn enable_forced_feature_packs(pack_repo: &mut PackRepository, flags: &FeatureFlagSet) {
    let requested_flags = pack_repo.get_requested_feature_flags();
    let flags_to_enable = flags.subtract(requested_flags);
    if flags_to_enable != FeatureFlagSet::empty() {
        todo!("Force enabling packs not yet supported")
    }
}

fn get_selected_packs(pack_repo: &PackRepository, include_disabled: bool) -> DataPackConfig {
    let enabled: Vec<String> = pack_repo.get_selected_ids().into_iter().cloned().collect();
    let disabled: Vec<String> = if include_disabled {
        pack_repo
            .available
            .keys()
            .filter(|id| !enabled.contains(*id))
            .cloned()
            .collect()
    } else {
        Vec::new()
    };
    DataPackConfig::new(enabled, disabled)
}
