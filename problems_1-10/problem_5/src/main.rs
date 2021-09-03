fn main() {
    let mut i: u32 = 1;

    'a: loop {
        for n in 1..=20 {
            if i % n == 0 {
                if n == 20 {
                    println!("{}", i);
                    break 'a;
                }
                continue;
            }
            break;
        }

        i += 1;
    }
}
