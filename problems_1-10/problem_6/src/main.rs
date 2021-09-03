fn main() {
    let mut a: u64 = 0;
    for i in 0..=100 {
        a += i * i;
    }

    let mut b: u64 = 0;
    for i in 0..=100 {
        b += i;
    }
    b *= b;

    println!("{}", b - a);
}
