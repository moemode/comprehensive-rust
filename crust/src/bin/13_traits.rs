trait Animal {
    fn leg_count(&self) -> u32;
}

trait Pet: Animal {
    fn talk(&self) -> String;
    fn greet(&self) {
        println!("Oh you're a cutie! What's your name? {}", self.talk());
    }
}

struct Dog {
    name: String,
    age: u8,
}

impl Animal for Dog {
    fn leg_count(&self) -> u32 {
        4
    }
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof, my name is {}!", self.name)
    }
}

#[derive(Debug)]
struct Meters(i32);
#[derive(Debug)]
struct MetersSquared(i32);

trait Multiply {
    type Output;
    fn multiply(&self, other: &Self) -> Self::Output;
}

impl Multiply for Meters {
    type Output = MetersSquared;
    
    fn multiply(&self, other: &Self) -> Self::Output {
        MetersSquared(self.0 * other.0)
    }
}

fn main() {
    let fido = Dog {name : String::from("Fido"), age: 5};
    fido.greet();
    println!("{} has {} legs", fido.name, fido.leg_count());

    let m1= Meters(3);
    let m2 = Meters(4);
    let area = m1.multiply(&m2);
    println!("Area: {:?}", area);
}