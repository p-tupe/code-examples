pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return vec![];
    }

    if garden[0].is_empty() {
        return vec!["".into()];
    }

    let grid: Vec<Vec<char>> = garden
        .iter()
        .map(|s| s.chars().collect())
        .collect::<Vec<_>>();

    let get_v = |x, y| grid.get(x as usize)?.get(y as usize);

    let mut resp = Vec::with_capacity(garden.len() * garden[0].len());

    for r in 0..grid.len() as isize {
        let mut str = String::with_capacity(r as usize);

        for c in 0..grid[0].len() as isize {
            let Some(v) = get_v(r, c) else {
                continue;
            };

            if v == &'*' {
                str.push('*');
                continue;
            }

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

            if s > 0 {
                str.push(s.to_string().chars().collect::<Vec<_>>()[0]);
            } else {
                str.push(*v);
            }
        }
        resp.push(str.to_string());
    }

    resp
}
