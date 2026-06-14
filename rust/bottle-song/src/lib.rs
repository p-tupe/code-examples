const SPELL_OUT: [&str; 11] = [
    "no", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
];

pub fn recite(mut start_bottles: u32, mut take_down: u32) -> String {
    let mut verse = String::new();

    while start_bottles > 0 && take_down > 0 {
        verse.push_str(&format!(
            "{starting} green bottle{ss} hanging on the wall,
{starting} green bottle{ss} hanging on the wall,
And if one green bottle should accidentally fall,
There'll be {remaining} green bottle{rs} hanging on the wall.\n\n",
            starting = SPELL_OUT[start_bottles as usize],
            remaining = SPELL_OUT[(start_bottles - 1) as usize].to_lowercase(),
            ss = if start_bottles > 1 { "s" } else { "" },
            rs = if start_bottles - 1 != 1 { "s" } else { "" }
        ));

        take_down -= 1;
        start_bottles -= 1;
    }

    verse.trim_end().to_string()
}
