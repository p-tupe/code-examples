//! Third solution: same as second, but functional
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|x| factors.iter().any(|&f| f != 0 && x.is_multiple_of(f)))
        .sum()
}
