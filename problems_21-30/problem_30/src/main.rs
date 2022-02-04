// Digit fifth powers
// https://projecteuler.net/problem=30

fn main() {
    let mut tot: u32 = 0;

    for n in 2..1_000_000 {
        let n_list: Vec<u32> = n.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();

        let mut sum = 0;
        
        for n_list_element in n_list {
            sum += n_list_element.pow(5);
        }

        if sum == n {
            tot += sum;
        }
    }

    println!("{tot}");
}
