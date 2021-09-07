fn main() {
    for t in 0..=u32::max_value() {
        let t: u32 = (t * (t + 1)) / 2;
        let mut divisors: u32 = 0;
        for d in 1..=(t / 2) + 1 {
            if t % d == 0 {
                divisors += 1;
            }
        }
        if divisors > 500 {
            println!("{}", t);
            break;
        }
    }
}
