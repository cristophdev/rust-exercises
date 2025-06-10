pub fn square(s: u32) -> u64 {
    1u64 << (s - 1)
}

pub fn total() -> u64 {
    let mut result = 1u64;
    for i in 2..=64 {
      result += square(i)
    }
    result
}