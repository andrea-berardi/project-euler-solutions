// Highly divisible triangular number
// https://projecteuler.net/problem=12

fn main() {
    let mut primes: Vec<u32> = Vec::with_capacity(100_000);

    for i in 0..=u32::max_value() {
        if i.is_prime() && primes.len() <= 100_000 {
            primes.push(i);
        } else if primes.len() <= 100_000 {
            continue;
        } else {
            break;
        }
    }

    let mut i: u32 = 2;
    let mut c: u32 = 0;
    let mut d: u32 = 2;
    let mut d1: u32 = 2;

    while c <= 500 {
        if i % 2 == 0 {
            d = count_divisors(i + 1, &primes);
        } else {
            d1 = count_divisors((i + 1) / 2, &primes);
        }
        c = d * d1;
        i += 1;
    }

    println!("{}", i * (i - 1) / 2);
}

fn count_divisors(n: u32, primes: &[u32]) -> u32 {
    let mut number_of_divisors: u32 = 1;
    let mut _exponent: u32 = 0;
    let mut remainder: u32 = n;

    for i in primes.iter().enumerate() {
        if i.1.pow(2) > n {
            return number_of_divisors * 2;
        }

        _exponent = 1;
        while remainder % i.1 == 0 {
            _exponent += 1;
            remainder /= i.1;
        }
        number_of_divisors *= _exponent;

        if remainder == 1 {
            return number_of_divisors;
        }
    }
    number_of_divisors
}

trait Prime {
    fn is_prime(&self) -> bool;
}

impl Prime for u32 {
    fn is_prime(&self) -> bool {
        if *self == 2 || *self == 3 {
            return true;
        }

        if *self <= 1 || *self % 2 == 0 || *self % 3 == 0 {
            return false;
        }

        let mut i: u32 = 5;
        while i * i <= *self {
            if *self % i == 0 || *self % (i + 2) == 0 {
                return false;
            }

            i += 6;
        }

        true
    }
}

// My first attempt (bruteforce) - NOT FINAL VERSION
// This takes ages to run, but it works

// fn main() {
//     for t in 0..=u32::max_value() {
//         let t: u32 = (t * (t + 1)) / 2;
//         let mut divisors: u32 = 0;
//         for d in 1..=(t / 2) + 1 {
//             if t % d == 0 {
//                 divisors += 1;
//             }
//         }
//         if divisors > 500 {
//             println!("{}", t);
//             break;
//         }
//     }
// }