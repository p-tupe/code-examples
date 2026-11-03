//! Second one inspired by community solns
pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        _ => collatz(if n % 2 == 0 { n / 2 } else { n * 3 + 1 }).map(|x| x + 1),
    }
}
