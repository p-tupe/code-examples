use clock::Clock;

fn main() {
    let c = Clock::new(10, 40);
    println!("{c}");
    println!("Add 1 min: {}", c.add_minutes(1));
    println!("Add 1 hour: {}", c.add_minutes(60));
    println!("Add 1 hour, 1 min: {}", c.add_minutes(61));
    println!("Add 24 hour: {}", c.add_minutes(24 * 60));
}
