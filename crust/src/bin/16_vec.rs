fn main() {
    let mut v = vec![0, 0, 1, 2, 3, 4];
    v.retain(|x| x % 2 == 0);
    println!("{:?}", v);
    v.dedup();
    println!("{:?}", v);
}