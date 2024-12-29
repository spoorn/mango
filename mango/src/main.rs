mod bootstrap;
mod core;
mod dedicated;
mod detected_version;
mod nbt;
mod packs;
mod resources;
mod shared_constants;
mod sounds;
mod util;
mod world;

use crate::dedicated::dedicated_server_settings::DedicatedServerSettings;
use crate::world::level::storage::level_storage_source::LevelStorageSource;
use tracing::{info, warn};

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
    let level_storage_access =
        level_storage_source.validate_and_create_access(properties.level_name.clone());

    if level_storage_access.has_world_data() {
        info!("Loading world data");
        match level_storage_access
            .get_data_tag(false, level_storage_source.get_data_fixer())
            .await
        {
            Ok(data_tag) => {
                info!(
                    "Loaded level data: {}",
                    serde_json::to_string_pretty(&data_tag.value).unwrap()
                );
                let level_summary = level_storage_access.make_level_summary(&data_tag, false);
            }
            Err(e) => {
                let level_directory = level_storage_access.level_directory;
                warn!(
                    ?e,
                    "Failed to load world data from {:?}",
                    level_directory.data_file()
                );
                info!("Attempting to use fallback");
            }
        }
    }
}
