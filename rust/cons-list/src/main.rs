#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn main() {
    println!("Cons List!");

    let nil: Box<_> = Box::new(List::Nil);
    let one = Box::new(List::Cons("one", nil));
    let two = Box::new(List::Cons("two", one));
    println!("{:?}", two);

    let another = List::Cons("some", Box::new(List::Nil));
    println!("{:?}", another);
}
