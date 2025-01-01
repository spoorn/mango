use crate::commands::commands::CommandSelection;
use crate::packs::repository::pack_repository::PackRepository;
use crate::packs::resources::resource_manager::ResourceManager;
use crate::world::level::world_data_configuration::WorldDataConfiguration;

// TODO: fill in params
pub async fn load(init_config: InitConfig) {}

// TODO: fill in fields
pub struct DataLoadContext {
    resources: Box<dyn ResourceManager>,
    data_configuration: WorldDataConfiguration,
}

#[derive(Debug)]
pub struct InitConfig {
    pack_config: PackConfig,
    command_selection: CommandSelection,
    function_compilation_level: u8,
}
impl InitConfig {
    pub fn new(
        pack_config: PackConfig,
        command_selection: CommandSelection,
        function_compilation_level: u8,
    ) -> Self {
        Self {
            pack_config,
            command_selection,
            function_compilation_level,
        }
    }
}

#[derive(Debug)]
pub struct PackConfig {
    pack_repository: PackRepository,
    initial_data_config: WorldDataConfiguration,
    safe_mode: bool,
    init_mode: bool,
}
impl PackConfig {
    pub fn new(
        pack_repository: PackRepository,
        initial_data_config: WorldDataConfiguration,
        safe_mode: bool,
        init_mode: bool,
    ) -> Self {
        Self {
            pack_repository,
            initial_data_config,
            safe_mode,
            init_mode,
        }
    }

    pub fn create_resource_manager() -> (WorldDataConfiguration, Box<dyn ResourceManager>) {
        todo!();
    }
}
