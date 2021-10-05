// Quadratic primes
// https://projecteuler.net/problem=27

fn main() {
    let (mut max_a, mut max_b, mut max_n) = (0_i32, 0_i32, 0_i32);

    for a in -1_000..=1_000 {
        for b in -1_000..=1_000 {
            let mut n: i32 = 0;

            while (n.pow(2) + a * n + b).is_prime() {
                n += 1;
            }

            if n > max_n {
                max_n = n;
                max_a = a;
                max_b = b;
            }
        }
    }

    println!("{}", max_a * max_b);
}

trait Prime {
    fn is_prime(&self) -> bool;
}

impl Prime for i32 {
    fn is_prime(&self) -> bool {
        if *self == 2 || *self == 3 {
            return true;
        }

        if *self <= 1 || *self % 2 == 0 || *self % 3 == 0 {
            return false;
        }

        let mut i: i32 = 5;
        while i * i <= *self {
            if *self % i == 0 || *self % (i + 2) == 0 {
                return false;
            }

            i += 6;
        }

        true
    }
}
