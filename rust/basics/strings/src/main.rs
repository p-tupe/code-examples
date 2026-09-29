fn main() {
    match "Hello World!".split_whitespace().next() {
        Some(word) => println!("First word: {}", word),
        None => return,
    }
}
