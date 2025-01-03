use crate::file_util;
use crate::packs::abstract_pack_resources;
use crate::packs::built_in_metadata::BuiltInMetadata;
use crate::packs::metadata::metadata_section_type::MetadataSectionType;
use crate::packs::metadata::pack::MetadataSection;
use crate::packs::pack_location_info::PackLocationInfo;
use crate::packs::pack_resources::PackResources;
use crate::packs::pack_type::{PackType, MC_ASSETS_ROOT_FILE};
use crate::resources::resource_location::ResourceLocation;
use include_dir::Dir;
use indexmap::IndexSet;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::LazyLock;
use strum::IntoEnumIterator;
use tracing::{debug, error};

// TODO: the asset files are empty in vanilla, why?
// It seems these don't get read anyways. I can't find any code that looks for `.mcassetsroot`
static ROOT_DIR_BY_TYPE: LazyLock<HashMap<PackType, DirEntry>> = LazyLock::new(|| {
    let res = PackType::iter()
        .filter_map(|pack_type| {
            let dir: &'static Dir = pack_type.get_directory();
            match dir.get_file(MC_ASSETS_ROOT_FILE) {
                None => {
                    error!(
                        "File {:?}/{} does not exist in binary path",
                        pack_type.get_directory().path(),
                        MC_ASSETS_ROOT_FILE
                    );
                    None
                }
                Some(_file) => {
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

                    // Save the parent paths of everywhere that has .mcassetsroot
                    Some((pack_type, DirEntry(dir)))
                }
            }
        })
        .collect();
    debug!("Loaded root pack resource asset dirs by type: {:?}", res);
    res
});

/// We include the asset directories directly into the binary and access them through include_dir's
/// [File] handler. We make it hashable here to help with deduping.
#[derive(Debug)]
pub struct DirEntry(&'static Dir<'static>);
impl Deref for DirEntry {
    type Target = &'static Dir<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Hash for DirEntry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path().hash(state);
    }
}
impl PartialEq for DirEntry {
    fn eq(&self, other: &Self) -> bool {
        self.path() == other.path()
    }
}
impl Eq for DirEntry {}

#[derive(Clone, Debug)]
pub struct VanillaPackResources {
    location: PackLocationInfo,
    metadata: BuiltInMetadata,
    namespaces: HashSet<String>,
    root_paths: Vec<&'static DirEntry>,
    paths_for_type: HashMap<PackType, Vec<&'static DirEntry>>,
}
impl VanillaPackResources {
    pub fn new(
        location: PackLocationInfo,
        metadata: BuiltInMetadata,
        namespaces: HashSet<String>,
        root_paths: Vec<&'static DirEntry>,
        paths_for_type: HashMap<PackType, Vec<&'static DirEntry>>,
    ) -> Self {
        Self {
            location,
            metadata,
            namespaces,
            root_paths,
            paths_for_type,
        }
    }

    pub fn list_raw_paths(
        &self,
        pack_type: PackType,
        pack_dir: &ResourceLocation,
        mut consumer: impl FnMut(Option<&'static include_dir::DirEntry>) -> (),
    ) {
        match file_util::decompose_path(&pack_dir.path) {
            Ok(path) => {
                if let Some(paths_for_type) = self.paths_for_type.get(&pack_type) {
                    let namespace = PathBuf::from(&pack_dir.namespace);
                    paths_for_type.iter().for_each(|paths| {
                        // include_dirs is a little clunky and requires specifying the full path
                        let entry = paths.get_entry(
                            paths.path().join(
                                namespace.join(file_util::resolve_path(
                                    path.iter()
                                        .map(|s| s.as_str())
                                        .collect::<Vec<&str>>()
                                        .as_slice(),
                                )),
                            ),
                        );
                        consumer(entry);
                    });
                }
            }
            Err(e) => error!("Invalid path {}: {:?}", pack_dir.path, e),
        };
    }
}
impl PackResources for VanillaPackResources {
    // TODO: The VanillaPackResources only holds root paths to
    fn get_root_resource(&self, parts: &[&str]) -> Option<&[u8]> {
        if let Err(e) = file_util::validate_path(parts) {
            panic!("Failed to validate paths: {:?}", e);
        }

        for root_path in &self.root_paths {
            // include_dirs is clunky and requires the full path
            if let Some(file) =
                root_path.get_file(root_path.path().join(file_util::resolve_path(parts)))
            {
                return Some(file.contents());
            }
        }

        None
    }

    fn get_metadata_section(
        &self,
        metadata_section_type: MetadataSectionType,
    ) -> Option<Rc<dyn MetadataSection>> {
        if let Some(mcmeta_data) = self.get_root_resource(&["pack.mcmeta"]) {
            match abstract_pack_resources::get_metadata_from_stream(
                &metadata_section_type,
                mcmeta_data,
            ) {
                None => (),
                Some(res) => return Some(Rc::from(res)),
            }
        }
        self.metadata.get(&(metadata_section_type)).map(Rc::clone)
    }
}

#[derive(Default)]
pub struct VanillaPackResourcesBuilder {
    metadata: BuiltInMetadata,
    namespaces: HashSet<String>,
    root_paths: IndexSet<&'static DirEntry>,
    paths_for_type: HashMap<PackType, IndexSet<&'static DirEntry>>,
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

    fn push_root_path(&mut self, path: &'static DirEntry) -> &mut Self {
        self.root_paths.insert(path);
        self
    }

    fn push_path_for_type(
        &mut self,
        pack_type: PackType,
        dir_file: &'static DirEntry,
    ) -> &mut Self {
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
