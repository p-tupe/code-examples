pub fn square(s: u32) -> u64 {
    2_u64.pow(s - 1)
}

pub fn total() -> u64 {
    (1..=64).reduce(|sum, s| sum + square(s as u32)).unwrap()
}
