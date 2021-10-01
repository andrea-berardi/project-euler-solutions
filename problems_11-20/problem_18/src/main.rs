// Maximum path sum I
// https://projecteuler.net/problem=18

fn main() {
    let file: String = std::fs::read_to_string("triangle.txt").unwrap();

    let mut triangle: Vec<Vec<u32>> = Vec::new();
    for line in file.lines() {
        triangle.push(
            line.split_ascii_whitespace()
                .map(|num: &str| num.parse::<u32>().unwrap())
                .collect(),
        );
    }

    for line in (0..triangle.len() - 1).rev() {
        for element in triangle[line].clone().iter().enumerate() {
            if triangle[line + 1][element.0] > triangle[line + 1][element.0 + 1] {
                triangle[line][element.0] += triangle[line + 1][element.0];
            } else {
                triangle[line][element.0] += triangle[line + 1][element.0 + 1];
            }
        }
    }

    println!("{}", triangle[0][0]);
}
