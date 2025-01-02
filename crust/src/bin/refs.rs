fn main() {
    let mut a = 'A';
    let mut b = 'B';
    let mut r= &mut a;
    *r = 'C';
    println!("r: {}", *r);
    r = &mut b;
    println!("r.is_ascii: {}", r.is_ascii());
    println!("r: {}", *r);
}
