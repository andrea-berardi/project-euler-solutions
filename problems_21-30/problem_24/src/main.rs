fn main() { // thanks Dijkstra
    let mut permutation: Vec<u32> = (0..=9).collect();
    let p_len: usize = permutation.len();

    let mut c: u32 = 0;
    while c <= 1_000_000 {
        let mut i: usize = p_len - 1;
        while permutation[i - 1] >= permutation[i] {
            i -= 1;
        }

        let mut j: usize = p_len;
        while permutation[j - 1] <= permutation[i - 1] {
            j -= 1;
        }

        permutation.swap(i - 1, j - 1);

        i += 1;
        j = p_len;

        while i < j {
            permutation.swap(i - 1, j - 1);

            i += 1;
            j -= 1;
        }

        c += 1;
    }

    for n in permutation {
        print!("{}", n);
    }
    println!();
}
