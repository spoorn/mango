use crate::resources::resource_location::ResourceLocation;

pub mod mth;

pub fn make_description_id(id: &str, location: ResourceLocation) -> String {
    format!(
        "{}.{}.{}",
        id,
        location.namespace,
        location.path.replace("/", ".")
    )
}
