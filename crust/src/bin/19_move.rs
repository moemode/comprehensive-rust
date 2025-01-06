#[derive(Debug, Clone, Copy)]
struct Point (i32, i32);

fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("s2: {s2}");
    // println!("s1: {s1}");
    let p1 = Point(1, 2);
    let p2 = p1;
    println!("p2: {:?}", p2);
    println!("p1: {:?}", p1);
}