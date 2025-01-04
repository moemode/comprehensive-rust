fn handle_input(c: char) {
    match c {
        'q' => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving"),
        '0'..='9' => println!("Number"),
        key if key.is_lowercase() => println!("Lowercase: {key}"),
        _ => println!("Unknown input"),
    }
}

fn main() {
    use std::io::{self, Write};
    loop {
        print!("Press a key (or 'q' to quit): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            let ch = input.trim().chars().next().unwrap_or('\n');
            if ch == 'q' {
                handle_input(ch);
                break;
            }
            handle_input(ch);
        }
    }
}
