// Factorial digit sum
// https://projecteuler.net/problem=20

fn main() {
    let mut factorial: num_bigint::BigUint = num_traits::One::one();

    for n in 1..=100 {
        factorial *= num_bigint::ToBigUint::to_biguint(&n).unwrap();
    }

    let factorial: Vec<u32> = factorial
        .to_string()
        .chars()
        .map(|c: char| c.to_digit(10).unwrap())
        .collect();

    println!("{}", factorial.iter().sum::<u32>());
}
