pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return vec![];
    }

    let grid: Vec<Vec<char>> = garden
        .iter()
        .map(|s| s.chars().collect())
        .collect::<Vec<_>>();

    grid.iter()
        .enumerate()
        .map(|(r, str)| {
            str.iter()
                .enumerate()
                .map(|(c, ch)| {
                    if *ch == '*' {
                        return *ch;
                    }

                    let s = (r.saturating_sub(1)..(r + 2).min(grid.len()))
                        .flat_map(|i| {
                            (c.saturating_sub(1)..(c + 2).min(str.len())).map(move |j| (i, j))
                        })
                        .filter(|&(i, j)| grid[i][j] == '*')
                        .count() as u8;

                    if s > 0 { (s + b'0') as char } else { *ch }
                })
                .collect()
        })
        .collect()
}
