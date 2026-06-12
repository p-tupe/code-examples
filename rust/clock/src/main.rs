use clock::Clock;

fn main() {
    let c = Clock::new(24, 0);
    let d = Clock::new(0, 0);
    println!("{c} == {d} = {}", c == d);
}
