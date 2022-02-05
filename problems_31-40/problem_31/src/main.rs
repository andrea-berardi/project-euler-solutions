// Coin sums
// https://projecteuler.net/problem=31

fn main() {
    let mut counter: usize = 1; // 2£ coin

    for one_pound in (0..=200).step_by(100) {
        for fifty_p in (0..=200).step_by(50) {
            for twenty_p in (0..=200).step_by(20) {
                for ten_p in (0..=200).step_by(10) {
                    for five_p in (0..=200).step_by(5) {
                        for two_p in (0..=200).step_by(2) {
                            for one_p in (0..=200).step_by(1) {
                                if one_pound + fifty_p + twenty_p + ten_p + five_p + two_p + one_p
                                    == 200
                                {
                                    counter += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{counter}");
}
