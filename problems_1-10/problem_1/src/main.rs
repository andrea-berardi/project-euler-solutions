// Multiples of 3 or 5
// https://projecteuler.net/problem=1

fn main() {
    let mut sum: u32 = 0;

    for i in 0..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    println!("{}", sum);
}
