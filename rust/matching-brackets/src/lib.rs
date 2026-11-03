//! Fourth attempt: Stack with spice
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut st = Vec::with_capacity(string.len());
    string.chars().all(|c| match c {
        '(' | '{' | '[' => {
            st.push(c);
            true
        }
        ']' | '}' | ')' => st.pop().map_or_else(
            || false,
            |x| matches!((x, c), ('(', ')') | ('{', '}') | ('[', ']')),
        ),
        _ => true,
    }) && st.is_empty()
}
