// Amicable numbers
// https://projecteuler.net/problem=21

fn main() {
    let mut total: u32 = 0;

    for a in 1..10_000 {
        if a == a.divisors_sum().divisors_sum() && a != a.divisors_sum() {
            total += a + a.divisors_sum();
        }
    }

    println!("{}", total / 2);
}

trait Divisors {
    fn divisors_sum(&self) -> u32;
}

impl Divisors for u32 {
    fn divisors_sum(&self) -> u32 {
        let mut divisors: Vec<u32> = Vec::new();

        for i in 1..(*self as f32).sqrt() as u32 {
            if *self % i == 0 {
                if *self / i == i {
                    divisors.push(i);
                } else {
                    divisors.push(i);

                    if *self != *self / i {
                        divisors.push(*self / i);
                    }
                }
            }
        }

        divisors.iter().sum()
    }
}
