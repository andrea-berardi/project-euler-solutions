// Longest Collatz sequence
// https://projecteuler.net/problem=14

fn main() {
    let mut _n: u64 = 0;
    let mut max: u64 = 0;
    let mut max_n: u64 = 0;

    for number in 1..1_000_000 {
        _n = number;
        let mut count: u64 = 0;

        while _n != 1 {
            if _n % 2 == 0 {
                _n /= 2;
            } else {
                _n = 3 * _n + 1;
            }

            count += 1;
        }

        if count > max {
            max = count;
            max_n = number;
        }
    }

    println!("{}", max_n);
}
