pub fn get_seed(x: i32, y: i32, z: i32) -> i64 {
    let mut v = x as i64 * 3129871 ^ y as i64 * 116129781 ^ z as i64;
    v = v * v * 42317861 + v * 11;
    v >> 16
}
