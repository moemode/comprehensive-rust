
enum Result {
    Ok(i32),
    Err(String),
}

fn divide_in_two(n: i32) -> Result {
    if n % 2 == 0 {
        Result::Ok(n / 2)
    } else {
        Result::Err(String::from("Not divisible by 2"))
    }
}

fn main() {
    let n = 100;
    match divide_in_two(n) {
        Result::Ok(x) => println!("{} divided by 2 is {}", n, x),
        Result::Err(e) => println!("Error: {}", e),
    }
}