fn main() {
    let s1: &str = "World";
    println!("s1: {s1}");
    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[s2.len() - s1.len()..];
    println!("s3: {s3}");

    // The format!() macro is a convenient way to generate an owned
    // string from dynamic values. It accepts the same format specification as println!().
    let s = format!("{} {}", s1, s3);
    println!("s: {s}");

    /* For C++ programmers: think of &str as std::string_view
    from C++, but the one that always points to a valid string
    in memory. Rust String is a rough equivalent of
    std::string from C++
    (main difference: it can only contain UTF-8 encoded bytes and
    will never use a small-string optimization). */
    println!(r#"<a href="link.html">link</a>"#);
}
