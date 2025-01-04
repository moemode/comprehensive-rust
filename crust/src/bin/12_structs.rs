struct Foo {
    x: (u32, u32),
    y: u32,
}

fn main() {
    let foo = Foo { x: (2,2), y: 2};
    let y_target = 4;
    match foo {
        Foo {x: (1, b), y} => println!("x.0 = 1, b = {b}, y = {y}"),
        Foo {y: y_target, x: i} => println!("y = {y_target}, x = {:?}", i),
        Foo {y, ..} => println!("y = {y}"),
    }
}
