// box ~ unique_ptr
/* Box<T> implements Deref<Target = T>, which means that you can call methods from T directly on a Box<T>. */

#[derive(Debug)]
enum List<T> {
    Nil,
    Element(T, Box<List<T>>)
}

fn main() {
    let five = Box::new(5);
    println!("five: {}", *five);
    let list = List::Element(1, Box::new(List::Element(2, Box::new(List::Nil))));
    println!("{list:?}");
}