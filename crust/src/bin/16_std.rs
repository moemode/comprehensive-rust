use std::{fs::File, io::Read, env};

fn main() {
    let name = "Mike Miller";
    let mut pos = name.find("ll");
    println!("find returned {pos:?}");
    assert_eq!(pos.unwrap(), 7);
    pos = name.find("M");
    println!("find returned {pos:?}");
    assert_eq!(pos.expect("Character not found"), 0);

    let f: Result<File, std::io::Error> = File::open("diary.txt");
    match f {
        Ok(mut f) => {
            let mut contents = String::new();
            if let Ok(bytes) = f.read_to_string(&mut contents) {
                println!("Dear diary: {contents} ({bytes} bytes)");
            } else {
                println!("Could not read file content");
            }
        }
        Err(err) => {
            println!("The diary could not be opened: {err}");
        }
    }
}