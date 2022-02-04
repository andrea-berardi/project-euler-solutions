fn main() {
    let mut counter: usize = 1; // 2£ coin

    for one_pound in 0..=2 {
        for fifty_p in 0..=4 {
            for twenty_p in 0..=10 {
                for ten_p in 0..=20 {
                    for five_p in 0..=40 {
                        for two_p in 0..=100 {
                            for one_p in 0..=200 {
                                if one_pound as f32
                                    + fifty_p as f32 / 2.0
                                    + twenty_p as f32 / 5.0
                                    + ten_p as f32 / 10.0
                                    + five_p as f32 / 20.0
                                    + two_p as f32 / 50.0
                                    + one_p as f32 / 100.0
                                    == 2.0
                                {
                                    counter += 1;

                                    // println!("1.00£ x {one_pound}");
                                    // println!("0.50£ x {fifty_p}");
                                    // println!("0.20£ x {twenty_p}");
                                    // println!("0.10£ x {ten_p}");
                                    // println!("0.05£ x {five_p}");
                                    // println!("0.02£ x {two_p}");
                                    // println!("0.01£ x {one_p}");
                                    // println!();

                                    // std::thread::sleep(std::time::Duration::from_millis(500));
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
