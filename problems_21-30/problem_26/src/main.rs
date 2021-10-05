// Reciprocal cycles
// https://projecteuler.net/problem=26

fn main() {
    let mut cycle_len: u32 = 0;

    for d in (2..=1_000).rev() {
        if cycle_len >= d {
            println!("{}", d + 1);
            break;
        }

        let mut cycle: Vec<u32> = vec![0; d as usize];
        let mut n: usize = 1;
        let mut pos: u32 = 0;

        while cycle[n] == 0 && n != 0 {
            cycle[n] = pos;
            n *= 10;
            n %= d as usize;
            pos += 1;
        }

        if pos - cycle[n] > cycle_len {
            cycle_len = pos - cycle[n];
        }
    }
}