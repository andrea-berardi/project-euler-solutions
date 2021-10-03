// 1000-digit Fibonacci number
// https://projecteuler.net/problem=25

fn main() {
    let mut first: num_bigint::BigUint = num_traits::Zero::zero();
    let mut second: num_bigint::BigUint = num_traits::One::one();

    for n in 1..u32::max_value() {
        let tmp: num_bigint::BigUint = first + &second;
        first = std::mem::replace(&mut second, tmp);

        if first.to_string().len() >= 1_000 {
            println!("{}", n);
            break;
        }
    }
}
