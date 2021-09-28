fn main() {
    const MAX: u32 = 28_123;

    let mut abundants: Vec<u32> = Vec::with_capacity(MAX as usize);

    for n in 1..=MAX {
        if n.is_abundant() {
            abundants.push(n);
        }
    }

    let mut is_sum_of_abundants: Vec<bool> = vec![false; MAX as usize + 1];
    for m in abundants.iter() {
        for n in abundants.iter() {
            if m + n <= MAX {
                is_sum_of_abundants[(m + n) as usize] = true;
            } else {
                break;
            }
        }
    }

    let mut total: u32 = 0;
    for i in is_sum_of_abundants.iter().enumerate() {
        if *i.1 == false {
            total += i.0 as u32;
        }
    }

    println!("{}", total);
}

trait Divisors {
    fn is_abundant(&self) -> bool;
}

impl Divisors for u32 {
    fn is_abundant(&self) -> bool {
        let mut divisors: Vec<u32> = Vec::new();

        for i in 1..*self {
            if *self % i == 0 {
                divisors.push(i);
            }
        }

        divisors.iter().sum::<u32>() > *self
    }
}
