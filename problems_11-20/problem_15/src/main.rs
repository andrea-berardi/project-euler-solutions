// this solution is theoretically correct, but the numbers are too big to fit in an unsigned 128bit integer
fn main() {
    const GRID: u128 = 20;

    println!("{}", (GRID * 2).factorial() / (GRID.factorial().pow(2)));
}

trait Factorial {
    fn factorial(&self) -> u128;
}

impl Factorial for u128 {
    fn factorial(&self) -> u128 {
        let mut factorial: u128 = 1;

        for n in 1..=*self {
            factorial *= n;
            println!("{}! = {}", n, factorial);
        }

        factorial
    }
}
