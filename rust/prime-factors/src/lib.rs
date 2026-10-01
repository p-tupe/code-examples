pub fn factors(n: u64) -> Vec<u64> {
    match (2..=n).find(|&f| n.is_multiple_of(f)) {
        Some(x) => [vec![x], factors(n / x)].concat(),
        None => vec![],
    }
}
