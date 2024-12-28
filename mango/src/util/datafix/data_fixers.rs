//! Data Fixer helps convert world data across versions to prevent breakage on existing worlds that
//! are on a different version than the current server.

use crate::shared_constants;
use crate::util::datafix::data_fix::DataFix;
use crate::util::datafix::schema::Schema;
use bon::Builder;
use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, LazyLock};

pub static DATA_FIXER: LazyLock<Arc<DataFixer>> = LazyLock::new(|| {
    // TODO: Add fixers
    Arc::new(
        DataFixer::builder()
            .data_version(shared_constants::WORLD_VERSION.world_version.version)
            .build(),
    )
});

#[derive(Builder, Debug)]
pub struct DataFixer {
    #[builder(field)]
    schemas: HashMap<u32, Schema>,
    #[builder(field)]
    global_list: Vec<DataFix>,
    #[builder(field)]
    fixer_versions: BTreeSet<u32>,
    data_version: u32,
}

pub fn get_data_fixer() -> Arc<DataFixer> {
    Arc::clone(&DATA_FIXER)
}
