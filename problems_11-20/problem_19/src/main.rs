// Counting Sundays
// https://projecteuler.net/problem=19

fn main() {
    let mut sundays: u32 = 0;

    let mut _julian_day: i64 = 0;

    for year in 1901..=2000 {
        for month in 1..=12 {
            _julian_day = ((month - 14) / 12)
                + (1461 * (year + 4800 + ((month - 14) / 12)) / 4)
                + (367 * (month - 2 - 12 * ((month - 14) / 12)) / 12)
                - (3 * (year + 4900 + ((month - 14) / 12)) / 400)
                + 1
                - 32075;

            if _julian_day % 7 == 0 {
                sundays += 1;
            }
        }
    }

    println!("{}", sundays);
}
