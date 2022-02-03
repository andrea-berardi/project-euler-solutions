// Number spiral diagonals
// https://projecteuler.net/problem=28

fn main() {
    println!("{}", compute_diag(500));
}

fn compute_diag(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }

    4 * (2 * n + 1).pow(2) - 12 * n + compute_diag(n - 1)
}