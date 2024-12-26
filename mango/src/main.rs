mod bootstrap;
mod core;
mod detected_version;
mod packs;
mod resources;
mod shared_constants;
mod sounds;
mod world;

use tracing::info;

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
}
