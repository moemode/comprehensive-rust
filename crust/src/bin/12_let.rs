use std::time::Duration;

fn slow_reverse(mut s: String) -> String {
    let mut r = String::new();
    while let Some(c) = s.pop() {
        r.push(c);
    }
    r
}

fn sleep_for(secs: f32) {
    if let Ok(duration) = Duration::try_from_secs_f32(secs) {
        std::thread::sleep(duration);
        println!("slept for {duration:?}");
    }
}

fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
    let Some(s) = maybe_string else {
        return Err(String::from("got None"));
    };
    let Some(first_byte_char) = s.chars().next() else {
        return Err(String::from("got empty string"));
    };
    let Some(digit) = first_byte_char.to_digit(16) else {
        return Err(String::from("not a hex digit"));
    };
    return Ok(digit);
}

fn main() {
    sleep_for(-10.0);
    sleep_for(0.8);
    let r = slow_reverse("Hello".to_string());
    println!("reversed: {r}");
}
