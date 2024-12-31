use crate::packs::built_in_metadata::BuiltInMetadata;
use crate::packs::pack_location_info::PackLocationInfo;
use crate::packs::pack_type::{PackType, MC_ASSETS_ROOT_FILE};
use include_dir::File;
use indexmap::IndexSet;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::path;
use std::path::PathBuf;
use std::sync::LazyLock;
use strum::IntoEnumIterator;
use tracing::{error, info, warn};
use url::Url;

// TODO: the asset files are empty in vanilla, why?
static ROOT_DIR_BY_TYPE: LazyLock<HashMap<PackType, DirFile>> = LazyLock::new(|| {
    let res = PackType::iter()
        .filter_map(|pack_type| {
            let dir = pack_type.get_directory();
            match dir.get_file(MC_ASSETS_ROOT_FILE) {
                None => {
                    error!(
                        "File {:?}/{} does not exist in binary path",
                        pack_type.get_directory().path(),
                        MC_ASSETS_ROOT_FILE
                    );
                    None
                }
                Some(file) => {
                    // TODO: no schema checks as we load the file into the binary which seems to be
                    // different than Java and we can't use paths here

                    // let abs_path = path::absolute(dir.path().join(file.path()))
                    //     .expect("Failed to convert file path to absolute");
                    // info!("abs path: {:?}", abs_path);
                    // let url =
                    //     Url::from_file_path(&abs_path).expect("Failed to convert file path to URL");
                    // let scheme = url.scheme();
                    // if "jar" != scheme && "file" != scheme {
                    //     warn!("Assets URL '{}' uses unexpected schema", url);
                    // }
                    Some((pack_type, DirFile(file)))
                }
            }
        })
        .collect();
    info!("Loaded root pack resource asset dirs by type: {:?}", res);
    res
});

/// We include the asset directories directly into the binary and access them through include_dir's
/// [File] handler. We make it hashable here to help with deduping.
#[derive(Debug, Eq, PartialEq)]
struct DirFile(&'static File<'static>);
impl Deref for DirFile {
    type Target = &'static File<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Hash for DirFile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path().hash(state);
    }
}

#[derive(Debug)]
pub struct VanillaPackResources {
    location: PackLocationInfo,
    metadata: BuiltInMetadata,
    namespaces: HashSet<String>,
    root_paths: Vec<&'static DirFile>,
    paths_for_type: HashMap<PackType, Vec<&'static DirFile>>,
}
impl VanillaPackResources {
    pub fn new(
        location: PackLocationInfo,
        metadata: BuiltInMetadata,
        namespaces: HashSet<String>,
        root_paths: Vec<&'static DirFile>,
        paths_for_type: HashMap<PackType, Vec<&'static DirFile>>,
    ) -> Self {
        Self {
            location,
            metadata,
            namespaces,
            root_paths,
            paths_for_type,
        }
    }
}

#[derive(Default)]
pub struct VanillaPackResourcesBuilder {
    metadata: BuiltInMetadata,
    namespaces: HashSet<String>,
    root_paths: IndexSet<&'static DirFile>,
    paths_for_type: HashMap<PackType, IndexSet<&'static DirFile>>,
}
// TODO: Some code in vanilla is not used but exists here. We can probably simplify things given that
impl VanillaPackResourcesBuilder {
    pub fn set_metadata(mut self, metadata: BuiltInMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn expose_namespace(mut self, namespaces: impl Iterator<Item = String>) -> Self {
        self.namespaces.extend(namespaces);
        self
    }

    // TODO: This does nothing in vanilla
    pub fn apply_development_config(self) -> Self {
        self
    }

    // Jar Jar Binks
    pub fn push_jar_resources(mut self) -> Self {
        ROOT_DIR_BY_TYPE.iter().for_each(|(pack_type, dir_file)| {
            self.push_root_path(dir_file);
            self.push_path_for_type(*pack_type, dir_file);
        });
        self
    }

    fn push_root_path(&mut self, path: &'static DirFile) -> &mut Self {
        self.root_paths.insert(path);
        self
    }

    fn push_path_for_type(&mut self, pack_type: PackType, dir_file: &'static DirFile) -> &mut Self {
        self.paths_for_type
            .entry(pack_type)
            .or_insert_with(IndexSet::new)
            .insert(dir_file);
        self
    }

    pub fn build(mut self, pack_location_info: PackLocationInfo) -> VanillaPackResources {
        self.root_paths.reverse();

        let paths_for_type = PackType::iter()
            .map(|pack_type| {
                (
                    pack_type,
                    self.paths_for_type
                        .remove(&pack_type)
                        .map(|paths| {
                            let mut paths = paths.clone();
                            paths.reverse();
                            paths
                        })
                        .unwrap_or_default()
                        .into_iter()
                        .collect(),
                )
            })
            .collect();

        VanillaPackResources::new(
            pack_location_info,
            self.metadata,
            self.namespaces,
            self.root_paths.into_iter().collect(),
            paths_for_type,
        )
    }
}

// We don't need to validate the dir path as we include these files directly into the binary
//fn validate_dir_path(_path: &File) -> bool {
// if !path.exists() {
//     return false;
// } else if !path.is_dir() {
//     panic!("Path {:?} is not a directory", path::absolute(path));
// }
//true
//}
