pub fn square(s: u32) -> u64 {
    (2 as u64).pow( (s- 1) as u32)
}

pub fn total() -> u64 {
    // println!("{}", square(64_u32 + 1_u32) - 1_u64);
    square(64_u32) - 1_u64 + square(64_u32)
}
