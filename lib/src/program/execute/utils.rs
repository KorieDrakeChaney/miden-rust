pub const U32_MAX: u64 = u32::MAX as u64;

pub fn max(a: u64, b: u64) -> u64 {
    if a > b {
        a
    } else {
        b
    }
}
