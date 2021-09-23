// Summation of primes
// https://projecteuler.net/problem=10

fn main() {
    let mut tot: u64 = 0;

    for i in 0..2_000_000 {
        if i.is_prime() {
            tot += i;
        }
    }

    println!("{}", tot);
}

trait Prime {
    fn is_prime(&self) -> bool;
}

impl Prime for u64 {
    fn is_prime(&self) -> bool {
        if *self == 2 || *self == 3 {
            return true;
        }

        if *self <= 1 || *self % 2 == 0 || *self % 3 == 0 {
            return false;
        }

        let mut i: u64 = 5;
        while i * i <= *self {
            if *self % i == 0 || *self % (i + 2) == 0 {
                return false;
            }

            i += 6;
        }

        true
    }
}
