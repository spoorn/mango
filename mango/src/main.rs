mod bootstrap;
mod codec;
mod core;
mod dedicated;
mod detected_version;
mod minecraft_server;
mod nbt;
mod network;
mod packs;
mod resources;
mod shared_constants;
mod sounds;
mod util;
mod world;

use crate::dedicated::dedicated_server_settings::DedicatedServerSettings;
use crate::packs::repository::server_packs_source;
use crate::world::level::storage::level_storage_source::LevelStorageSource;
use tracing::{error, info, warn};

async fn setup_logging() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting tracing subscriber failed");
}

#[tokio::main]
async fn main() {
    setup_logging().await;
    info!("Hello, world!");
    info!("World version: {:#?}", *shared_constants::WORLD_VERSION);

    // TODO: Crash Reports
    // TODO: log file
    // TODO: profiling

    bootstrap::bootstrap();
    // TODO: validate bootstrap including missing translations, commands, Attribute suppliers for entities
    // TODO: timer hack thread
    let properties = DedicatedServerSettings::new("server.properties");
    info!("Loaded server properties: {:#?}", properties);
    properties.force_save();
    // TODO: RegionFileVersion, EULA, YggdrasilAuthenticationService
    let level_storage_source = LevelStorageSource::create_default(properties.universe.clone());
    let mut level_storage_access =
        level_storage_source.validate_and_create_access(properties.level_name.clone());

    let mut level_data = None;
    if level_storage_access.has_world_data() {
        info!("Loading world data");
        // Note: We don't catch panics here like vanilla and instead rely on returning errors. We'll
        // add error returns as we find any missing ones.
        level_data = match level_storage_access
            .get_data_tag(false, level_storage_source.get_data_fixer())
            .await
        {
            Ok(data_tag) => {
                info!(
                    "Loaded level data: {}",
                    serde_json::to_string_pretty(&data_tag.value).unwrap()
                );
                let level_summary = level_storage_access.make_level_summary(&data_tag, false);
                info!(
                    "Level summary: {}",
                    serde_json::to_string_pretty(&level_summary).unwrap()
                );
                Some((data_tag, level_summary))
            }
            Err(e) => {
                warn!(
                    ?e,
                    "Failed to load world data from {:?}",
                    level_storage_access.level_directory.data_file()
                );
                warn!("Attempting to use fallback");

                match level_storage_access
                    .get_data_tag(true, level_storage_source.get_data_fixer())
                    .await
                {
                    Ok(data_tag) => {
                        info!(
                            "Loaded fallback level data: {}",
                            serde_json::to_string_pretty(&data_tag.value).unwrap()
                        );
                        let level_summary =
                            level_storage_access.make_level_summary(&data_tag, false);
                        info!(
                            "Fallback Level summary: {}",
                            serde_json::to_string_pretty(&level_summary).unwrap()
                        );
                        level_storage_access.restore_level_data_from_old();
                        Some((data_tag, level_summary))
                    }
                    Err(e) => {
                        error!(
                            ?e,
                            "Failed to load fallback world data from {:?}",
                            level_storage_access.level_directory.old_data_file()
                        );
                        error!("Failed to load world data from {:?} and {:?}. World files may be corrupted. Shutting down.",
                            level_storage_access.level_directory.data_file(),
                            level_storage_access.level_directory.old_data_file()
                        );
                        None
                    }
                }
            }
        };

        if level_data
            .as_ref()
            .is_some_and(|(_, level_summary)| level_summary.requires_manual_conversion)
        {
            error!(
                "This world must be opened in an older version (like 1.6.4) to be safely converted"
            );
            return;
        }

        if !level_data
            .as_ref()
            .is_some_and(|(_, level_summary)| level_summary.is_compatible())
        {
            error!("This world was created by an incompatible version.");
            return;
        }
    }

    // TODO: handle safe mode command line arg
    if properties.safe_mode {
        warn!("Safe mode active, only vanilla datapack will be loaded");
    }

    let pack_repository = server_packs_source::create_pack_repository(
        &level_storage_source,
        &mut level_storage_access,
    );
    info!("Pack Repository: {:#?}", pack_repository);
}
