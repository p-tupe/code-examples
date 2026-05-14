use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&'a str> = HashSet::new();

    let mut word_chars: Vec<char> = word.to_lowercase().chars().collect();
    word_chars.sort_unstable();

    for candidate in possible_anagrams {
        if word.to_lowercase() == candidate.to_lowercase() || word.len() != candidate.len() {
            continue;
        }

        let mut candidate_chars: Vec<char> = candidate.to_lowercase().chars().collect();
        candidate_chars.sort_unstable();

        if word_chars == candidate_chars {
            anagrams.insert(candidate);
        }
    }

    anagrams
}
