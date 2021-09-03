fn main() {
    let mut n: u64 = 600_851_475_143;
    let mut max_prime: u64 = 0;

    if n % 2 == 0 {
        max_prime = 2;

        while n % 2 == 0 {
            n /= 2;
        }
    }

    let mut _j: u64 = 0;
    for i in (3..(n as f64).sqrt() as u64).step_by((_j + 2) as usize) {
        if n % i == 0 {
            max_prime = i;
        }

        while n % i == 0 {
            n /= i;
        }

        _j = i;
    }

    if n > max_prime {
        max_prime = n;
    }

    println!("{}", max_prime);
}
