use std::{
    borrow::BorrowMut,
    cell::{Cell, RefCell},
};

fn main() {
    /*
    Cell wraps a value and allows getting or setting the value using only a shared reference to the Cell.
    However, it does not allow any references to the inner value.
    Since there are no references, borrowing rules cannot be broken.
    */
    let cell = Cell::new(5);
    cell.set(123);
    println!("{}", cell.get());

    /*
    RefCell allows accessing and mutating a wrapped value by providing alternative types Ref and RefMut 
    that emulate &T/&mut T without actually being Rust references.
    These types perform dynamic checks using a counter in the RefCell to prevent existence of a RefMut 
    alongside another Ref/RefMut.
    By implementing Deref (and DerefMut for RefMut), these types allow calling methods on the inner value 
    without allowing references to escape.
    */
    let rcell = RefCell::new(5);
    {
        let mut cell_ref = rcell.borrow_mut();
        *cell_ref = 10;
        // let other = rcell.borrow();
        // println!("{}", *other);
    }
    println!("{rcell:?}");
}
