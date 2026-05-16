//! Second try, using windows
//! Refer https://exercism.org/tracks/rust/exercises/sublist/solutions/LudwigStecher
use std::cmp::Ordering::{Equal, Greater, Less};

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    match first_list.len().cmp(&second_list.len()) {
        Equal if first_list == second_list => Comparison::Equal,
        Less if includes(second_list, first_list) => Comparison::Sublist,
        Greater if includes(first_list, second_list) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}

/// Does larger l include smaller s?
fn includes(l: &[i32], s: &[i32]) -> bool {
    s.is_empty() || l.windows(s.len()).any(|w| w == s)
}
