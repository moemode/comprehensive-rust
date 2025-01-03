fn main() {
    /* In this example, r is mutable so that it can be 
    reassigned (r = &b). Note that this re-binds r, so that 
    it refers to something else. 
    This is different from C++, where assignment to a 
    reference changes the referenced value. */
    let a = 'A';
    let b = 'B';
    let mut r: &char = &a;
    println!("r: {}", *r);
    r = &b;
    println!("r: {}", *r);
    let mut point = (1, 2);
    let x_coord = &mut point.0;
    let y_coord = &mut point.1;
    *x_coord = 20;
    println!("x_coord: {x_coord}");
    println!("y_coord: {y_coord}");
    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {:?}", a);
    let s = &a[2..4];
    println!("s: {s:?}");
}
