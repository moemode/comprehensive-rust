#[derive(Debug, Default)]
struct Derived {
    x: u32,
    y: String,
    z: Implemented,
}

#[derive(Debug)]
struct Implemented(String);

impl Default for Implemented {
    fn default() -> Self {
        Self("John Smith".into())
    }
}

fn main () {
    let almost_default = Derived {y: "Hello".into(), ..Default::default()};
    println!("{:?}", almost_default);
}