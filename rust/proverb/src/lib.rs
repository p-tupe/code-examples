// Adapted from https://exercism.org/tracks/rust/exercises/proverb/solutions/nickers
use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    list.first()
        .map(|&first| {
            list.windows(2)
                .map(|words| format!("For want of a {} the {} was lost.", words[0], words[1]))
                .chain(once(format!("And all for the want of a {}.", first)))
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or("".into())
}
