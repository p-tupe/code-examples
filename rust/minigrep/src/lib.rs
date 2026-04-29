pub fn search<'a>(query: &str, contents: &'a str, lowercase: bool) -> Vec<&'a str> {
    if lowercase {
        contents
            .lines()
            .filter(|l| l.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    } else {
        contents.lines().filter(|l| l.contains(query)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, false)
        )
    }
    #[test]

    fn case_isensitive() {
        let query = "dUcT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, true)
        )
    }
}
