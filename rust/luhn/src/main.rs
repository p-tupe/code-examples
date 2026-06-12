use luhn::is_valid;

fn main() {
    let x = is_valid("055 444 285");
    print!("{x}");
}
