struct Person {
    age: u8,
    name: String,
}

struct Point (i32, i32);
struct Newtons(f64);

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,
    Run(Direction),
    Teleport { x: u32, y: u32},
}

fn describe(person: &Person) {
    println!("{} is {} years old", person.name, person.age);
}

enum Bar {
    A,
    B = 10000,
    C,
}

fn main() {
    let mut p = Person {name: String::from("Peter"), age: 27};
    describe(&p);
    let jackie = Person {name: String::from("Jackie"), ..p};
    describe(&jackie);
    let mut f = Newtons(9.81);
    f.0 += 1.0;
    println!("f: {}", f.0);
    let player_move = PlayerMove::Run(Direction::Left);
    println!("{:?}", player_move);
    println!("A: {}", Bar::A as u32);
    println!("B: {}", Bar::B as u32);
    println!("C: {}", Bar::C as u32);
}