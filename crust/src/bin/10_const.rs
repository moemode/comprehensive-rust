use std::sync::Mutex;

const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

static BANNER: Mutex<&'static str> = Mutex::new("Welcome to Crust!");

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

fn main() {
    let d = compute_digest("Hello");
    println!("digest: {d:?}");
    {
        let mut banner_guard = BANNER.lock().unwrap();
        *banner_guard = "Changed banner in main";
    }
    println!("BANNER: {:?}", BANNER.lock().unwrap());
}