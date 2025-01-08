use std::collections::{HashSet, VecDeque};

fn main() {
    let result: i32 = (1..=10).filter(|&x| x % 2 == 0).map(|x| x * x).sum();
    println!("The sum of squares of even numbers from 1 to 10 is: {}", result);
    let primes = [2, 3, 5, 7];
    let prime_squares = primes.into_iter().map(|x| x * x).collect::<VecDeque<_>>();
    println!("The squares of prime numbers are: {:?}", prime_squares);
}
