pub fn nth(n: u32) -> u32 {
    (2..).filter(is_prime).nth(n as usize).unwrap()
}

/// A number n is prime if
/// it is not divisible by 2 or 3 and
/// it is not divisible by any number
/// 6k-1 or 6k+1 where 5 <= k <= root n
fn is_prime(n: &u32) -> bool {
    match n {
        2 | 3 => true,
        n if n % 2 == 0 || n % 3 == 0 => false,
        n => !(5..)
            .step_by(6)
            .take_while(|i| i * i <= *n)
            .any(|i| n % i == 0 || n % (i + 2) == 0),
    }
}
