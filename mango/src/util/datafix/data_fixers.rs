//! Data Fixer helps convert world data across versions to prevent breakage on existing worlds that
//! are on a different version than the current server.

use crate::shared_constants;
use crate::util::datafix::data_fix::DataFix;
use crate::util::datafix::data_fix_types::DataFixTypes;
use crate::util::datafix::schema::Schema;
use crate::util::datafix::serialization::dynamic::Dynamic;
use bon::Builder;
use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, LazyLock};

pub static DATA_FIXER: LazyLock<Arc<DataFixer>> = LazyLock::new(|| {
    // TODO: Add fixers
    Arc::new(
        DataFixer::builder()
            .data_version(shared_constants::get_current_data_version())
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
    data_version: i32,
}
impl DataFixer {
    pub fn update<T>(
        &self,
        data_fix_type: &DataFixTypes,
        input: Dynamic<T>,
        version: i32,
        new_version: i32,
    ) -> Dynamic<T> {
        if version < new_version {
            todo!();
        } else {
            input
        }
    }
}

pub fn get_data_fixer() -> Arc<DataFixer> {
    Arc::clone(&DATA_FIXER)
}
