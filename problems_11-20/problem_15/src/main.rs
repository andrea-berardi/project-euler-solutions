// Lattice paths
// https://projecteuler.net/problem=15

fn main() {
    const GRID: u64 = 20;

    let mut paths: u64 = 1; // 1 is the neutral element of multiplication
    for i in 0..GRID {
        paths *= (GRID * 2) - i;
        paths /= i + 1;
    }
    println!("{}", paths);
}

// this solution is theoretically correct, but the numbers are too big to fit in an unsigned 128bit integer
// fn main() {
//     const GRID: u128 = 20;
// 
//     println!("{}", (GRID * 2).factorial() / (GRID.factorial().pow(2)));
// }
// 
// trait Factorial {
//     fn factorial(&self) -> u128;
// }
// 
// impl Factorial for u128 {
//     fn factorial(&self) -> u128 {
//         let mut factorial: u128 = 1;
// 
//         for n in 1..=*self {
//             factorial *= n;
//         }
// 
//         factorial
//     }
// }
