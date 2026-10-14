pub fn raindrops(n: u32) -> String {
    match [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .map(|(k, v)| if n.is_multiple_of(k) { v } else { "" })
        .join("")
    {
        s if s.is_empty() => n.to_string(),
        s => s,
    }
}
