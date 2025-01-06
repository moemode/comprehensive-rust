use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    next: RefCell<Option<Rc<Node>>>,
    prev: RefCell<Option<Weak<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<Node> {
        Rc::new(Node {
            value,
            next: RefCell::new(None),
            prev: RefCell::new(None),
        })
    }
}

fn main() {
    let a = Node::new(5);
    let b = Node::new(10);

    // a strongly references b
    *a.next.borrow_mut() = Some(Rc::clone(&b));

    // b weakly references a
    *b.prev.borrow_mut() = Some(Rc::downgrade(&a));

    println!("a strong count: {}", Rc::strong_count(&a)); // 1
    println!("b strong count: {}", Rc::strong_count(&b)); // 2

    // Dropping these Rc nodes breaks the cycle because 'b' only holds a Weak to 'a'
}