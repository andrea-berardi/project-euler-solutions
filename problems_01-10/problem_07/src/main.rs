// 10001st prime
// https://projecteuler.net/problem=7

fn main() {
    let mut c: u32 = 0;

    for i in 0..=u32::max_value() {
        if i.is_prime() {
            c += 1;

            if c == 10_001 {
                println!("{}", i);
                break;
            }
        }
    }
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
