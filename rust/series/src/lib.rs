//! Second version, inspired by community solns
pub fn series(digits: &str, len: usize) -> Vec<String> {
    (len..=digits.len()).map(|w| digits[(w - len)..w].into()).collect()
}
