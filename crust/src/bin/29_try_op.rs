use std::{
    fs,
    io::{self, Read},
};

fn read_username(path: &str) -> Result<String, io::Error> {
    let mut username_file = fs::File::open(path)?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    // fs::write("config.dat", "").unwrap();
    let username = read_username("config.dat");
    println!("username or error: {username:?}");
}
