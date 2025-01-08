/*
Iteration needs
* state tracking current position
* termination condition
* update state to move to next position
* way to access value at current position
*/
struct SliceIter<'a, T> {
    slice: &'a [T],
    index: usize,
}

impl<'a, T> Iterator for SliceIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.slice.len() {
            None
        } else {
            let next = &self.slice[self.index];
            self.index += 1;
            Some(next)
        }
    }
}

fn main() {
    // Example with integers
    let int_slice = [2, 4, 6, 8].as_slice();
    let iter = SliceIter { slice: int_slice, index: 0 };
    for elem in iter {
        println!("elem: {elem}");
    }

    // Example with strings
    let str_slice = ["hello", "world"].as_slice();
    let iter = SliceIter { slice: str_slice, index: 0 };
    for elem in iter {
        println!("elem: {elem}");
    }
}