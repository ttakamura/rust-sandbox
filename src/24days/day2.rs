extern crate primal;

use primal::Sieve;

fn num_divisors(n: usize, primes: &Sieve) -> Option<usize> {
    match primes.factor(n) {
        Ok(factors) => Some(factors.into_iter().fold(1, |acc, (_, x)| acc * (x+1))),
        Err(_)      => None,
    }
}

fn main() {
    let sieve = Sieve::new(10000);
    let suspect = 5273;
    println!("{} is prime: {}", suspect, sieve.is_prime(suspect));

    for num in sieve.primes_from(0).take(10) {
        print!("{}, ", num);
    }
    println!("");

    match sieve.primes_from(0).nth(10) {
        Some(number) => println!("{} is 10th prime number", number),
        None => println!("I don't know 10th prime number")
    }

    println!("{:?}", num_divisors(101, &sieve));
}

// cargo run --bin day2
