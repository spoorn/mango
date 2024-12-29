use crate::shared_constants;
use crate::util::datafix::data_fixers::DataFixer;
use crate::util::datafix::serialization::dynamic::Dynamic;

pub enum DataFixTypes {
    Level,
    Player,
    Chunk,
    HotBar,
    Options,
    Structure,
    Stats,
    SavedDataCommandStorage,
    SavedDataForcedChunks,
    SavedDataMapData,
    SavedDataMapIndex,
    SavedDataRaids,
    SavedDataRandomSequences,
    SavedDataScoreboard,
    SavedDataStructureFeatureIndices,
    Advancements,
    PoiChunk,
    WorldGenSettings,
    EntityChunk,
}
impl DataFixTypes {
    pub fn update<T>(
        &self,
        data_fixer: &DataFixer,
        input: Dynamic<T>,
        version: i32,
        new_version: i32,
    ) -> Dynamic<T> {
        data_fixer.update(self, input, version, new_version)
    }

    pub fn update_to_current_version<T>(
        &self,
        data_fixer: &DataFixer,
        input: Dynamic<T>,
        version: i32,
    ) -> Dynamic<T> {
        self.update(
            data_fixer,
            input,
            version,
            shared_constants::get_current_data_version(),
        )
    }
}
