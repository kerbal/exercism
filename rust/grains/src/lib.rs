pub fn square(s: u32) -> u64 {
    if 1 <= s && s <= 64 {
        return u64::pow(2, s - 1)
    }
    panic!("Square must be between 1 and 64")
}

pub fn total() -> u64 {
    (1..=64).map(|x| square(x)).sum()
}
