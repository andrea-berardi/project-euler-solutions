// Largest palindrome product
// https://projecteuler.net/problem=4

fn main() {
    let mut biggest_palindrome: u32 = 0;

    for a in 0..=999 {
        for b in 0..=999 {
            if (a * b).is_palindrome() && a * b > biggest_palindrome {
                biggest_palindrome = a * b;
            }
        }
    }

    println!("{}", biggest_palindrome);
}

trait Palindrome {
    fn is_palindrome(&self) -> bool;
}

impl Palindrome for u32 {
    fn is_palindrome(&self) -> bool {
        let s: Vec<char> = self.to_string().chars().collect();
        let mut rev_s: Vec<char> = s.clone();
        rev_s.reverse();

        s == rev_s
    }
}
