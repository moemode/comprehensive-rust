use std::cmp::Ord;
use std::cmp::Ordering;

fn min<T:Ord>(a: T, b: T) -> T {
    match b.cmp(&a) {
        Ordering::Equal | Ordering::Less => b,
        Ordering::Greater => a
    }
}

fn main() {
    assert_eq!(min(0, 10), 0);
    assert_eq!(min(500, 123), 123);

    assert_eq!(min('a', 'z'), 'a');
    assert_eq!(min('7', '1'), '1');

    assert_eq!(min("hello", "goodbye"), "goodbye");
    assert_eq!(min("bat", "armadillo"), "armadillo");
}