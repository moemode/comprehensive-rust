
fn main() {
    let mut s1 = String::from("Hello");
    s1.push_str(" ");
    s1.push_str("World");
    unsafe {
        let (capacity, ptr, len): (usize, usize, usize) = std::mem::transmute(s1);
        println!("capacity = {capacity}, ptr = {ptr:#x}, len = {len}");
    }
}