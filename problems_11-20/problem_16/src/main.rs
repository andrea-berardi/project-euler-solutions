// Power digit sum
// https://projecteuler.net/problem=16

fn main() {
    const EXP: usize = 1000;

    let mut number: Vec<u32> = Vec::with_capacity((1.0 + EXP as f32 * 2.0_f32.log(10.0)) as usize);
    number.push(1);

    for _ in 0..EXP {
        let mut carry: u32 = 0;
        let mut _prod: u32 = 0;

        for n in number.iter_mut() {
            _prod = 2 * *n + carry;
            *n = _prod % 10;
            carry = _prod / 10;
        }

        if carry != 0 {
            number.push(carry);
        }
    }

    println!("{}", number.iter().sum::<u32>());
}
