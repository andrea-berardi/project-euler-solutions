// Distinct powers
// https://projecteuler.net/problem=29

fn main() {
    let mut m: std::collections::HashMap<num_bigint::BigUint, bool> =
        std::collections::HashMap::new();

    for a in 2..=100u32 {
        for b in 2..=100u32 {
            let a: num_bigint::BigUint = a.into();
            let n: num_bigint::BigUint = a.pow(b);

            m.insert(n, true);
        }
    }

    println!("{}", m.len());
}
