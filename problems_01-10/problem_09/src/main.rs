// Special Pythagorean triplet
// https://projecteuler.net/problem=9

fn main() {
    let (mut _a, mut _b, mut _c): (u32, u32, u32) = (0, 0, 0);

    let mut m: u32 = 2;

    'a: loop {
        for n in 1..m {
            _a = m * m - n * n;
            _b = 2 * m * n;
            _c = m * m + n * n;

            if _a + _b + _c == 1_000 {
                println!("{}", _a * _b * _c);
                break 'a;
            }
        }

        m += 1;
    }
}
