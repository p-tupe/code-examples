pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return vec![];
    }

    let grid: Vec<Vec<char>> = garden
        .iter()
        .map(|s| s.chars().collect())
        .collect::<Vec<_>>();

    let get_v = |x, y| {
        grid.get(x as usize)
            .and_then(|str: &Vec<char>| str.get(y as usize))
    };

    grid.iter()
        .enumerate()
        .map(|(r, str)| {
            str.iter()
                .enumerate()
                .map(|(c, ch)| {
                    if *ch == '*' {
                        return ch.to_string();
                    }

                    // TODO: This bit hacky
                    let r = r as isize;
                    let c = c as isize;

                    let s = [
                        get_v(r - 1, c - 1), // north-west
                        get_v(r - 1, c),     // north
                        get_v(r - 1, c + 1), // north-east
                        get_v(r, c + 1),     // east
                        get_v(r + 1, c + 1), // south-east
                        get_v(r + 1, c),     // south
                        get_v(r + 1, c - 1), // south-west
                        get_v(r, c - 1),     // west
                    ]
                    .iter()
                    .map(|c| c.map_or_else(|| 0, |x| if *x == '*' { 1 } else { 0 }))
                    .sum::<isize>();

                    if s > 0 { s.to_string() } else { ch.to_string() }
                })
                .collect::<String>()
        })
        .collect()
}
