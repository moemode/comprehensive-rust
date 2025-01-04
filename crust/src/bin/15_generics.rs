
fn pick<T>(n: i32, even: T, odd: T) -> T {
    if n % 2 == 0 {
        even
    } else {
        odd
    }
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn coords(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }

    fn set_x(&mut self, x: T) {
        self.x = x;
    }
}

trait Printer {
    fn print(&self);
}

struct Wrapper<T>(T);

impl<T> Printer for Wrapper<T> {
    fn print(&self) {
        println!("Wrapper");
    }
}

fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

fn add_42_millions(x: impl Into<i32>) -> i32 {
    x.into() + 42_000_000
}

fn pair_of(x: u32) -> impl std::fmt::Debug {
    (x + 1, x - 1)
}

fn main() {
    println!("picked a number: {:?}", pick(97, 222, 333));
    println!("picked a string: {:?}", pick(28, "dog", "cat"));
    let ip = Point {x: 5, y: 10};
    let fp = Point {x: 1.0, y: 4.0};
    println!("{ip:?} and {fp:?}");
    println!("coords: {:?}", ip.coords());
    let w = Wrapper(109);
    w.print();
    let foo = String::from("foo");
    let pair = duplicate(&foo);
    println!("{pair:?}");
    let many = add_42_millions(42_i8);
    println!("{many}");
    let many_more = add_42_millions(10_000_000);
    let debuggable: () = pair_of(27);
    println!("debuggable: {debuggable:?}");
}