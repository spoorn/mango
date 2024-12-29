use crate::nbt::compound_tag::CompoundTag;

pub fn get_data_version(compound_tag: &CompoundTag, default: i32) -> i32 {
    let res = compound_tag.get_int("DataVersion");
    if res == 0 {
        default
    } else {
        res
    }
}
