//! Third attempt: Slighly faster and readable
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut s: String = string.chars().filter(|&c| ("[{()}]").contains(c)).collect();
    while !s.is_empty() {
        let l = s.len();
        s = s.replace("[]", "").replace("{}", "").replace("()", "");
        if l == s.len() {
            return false;
        }
    }
    true
}
