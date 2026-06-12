use flower_field::annotate;

fn main() {
    let x = annotate(&["·*·*·", "··*··", "··*··", "·····"]);
    println!("{x:#?}");
}
