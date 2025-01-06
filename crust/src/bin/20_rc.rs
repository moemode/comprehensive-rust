use std::rc::Rc;
// Rc ~ shared_pointer

fn main() {
    let a = Rc::new(10);
    let b = Rc::clone(&a);
    println!("a: {a}");
    println!("b: {b}");
}