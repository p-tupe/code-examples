#[derive(Debug)]
struct Rect {
    base: u32,
    height: u32,
}

impl Rect {
    fn square(size: u32) -> Rect {
        Rect {
            base: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.base * self.height
    }
}

fn main() {
    let r = Rect { base: 2, height: 5 };
    let a = r.area();
    println!("Area of {r:?}: {a}");
    println!("And square {:?}", Rect::square(2))
}
