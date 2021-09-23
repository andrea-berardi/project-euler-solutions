fn main() {
    let file: String = std::fs::read_to_string("names.txt").unwrap();
    let mut file: Vec<String> = file
        .trim()
        .split(',')
        .map(|name: &str| name.replace("\"", ""))
        .collect();

    file.sort_unstable();

    let mut total: u32 = 0;

    for name in file.iter().enumerate() {
        let mut tmp: u32 = 0;

        for c in name.1.chars() {
            tmp += c.c_index();
            println!("{}: {}", c, c.c_index());
        }

        total += tmp * (name.0 as u32 + 1);
    }

    println!("{}", total);
}

trait Letter {
    fn c_index(&self) -> u32;
}

impl Letter for char {
    fn c_index(&self) -> u32 {
        match &self.to_ascii_lowercase() {
            'a' => 1,
            'b' => 2,
            'c' => 3,
            'd' => 4,
            'e' => 5,
            'f' => 6,
            'g' => 7,
            'h' => 8,
            'i' => 9,
            'j' => 10,
            'k' => 11,
            'l' => 12,
            'm' => 13,
            'n' => 14,
            'o' => 15,
            'p' => 16,
            'q' => 17,
            'r' => 18,
            's' => 19,
            't' => 20,
            'u' => 21,
            'v' => 22,
            'w' => 23,
            'x' => 24,
            'y' => 25,
            'z' => 26,
            _ => 0,
        }
    }
}
