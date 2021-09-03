fn main() {
    let mut fibonacci: u32 = 0;
    let mut first: u32 = 0;
    let mut second: u32 = 1;
    let mut sum: u32 = 0;

    for _ in 2..=u32::max_value() {
        if fibonacci < 4_000_000 {
            fibonacci = first + second;
            first = second;
            second = fibonacci;

            if fibonacci % 2 == 0 {
                sum += fibonacci;
            }
        } else {
            break;
        }
    }

    println!("{}", sum);
}
