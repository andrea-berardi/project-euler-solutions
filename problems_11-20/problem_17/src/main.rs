fn main() {
    let mut tot: u32 = 0;

    for n in 1..=1000 {
        tot += n.in_english();
    }

    println!("{}", tot);
}

trait English {
    fn in_english(&self) -> u32;
}

impl English for u32 {
    fn in_english(&self) -> u32 {
        let mut n: u32 = *self;

        let n_0_19: [u32; 20] = [
            0,
            "one".len() as u32,
            "two".len() as u32,
            "three".len() as u32,
            "four".len() as u32,
            "five".len() as u32,
            "six".len() as u32,
            "seven".len() as u32,
            "eight".len() as u32,
            "nine".len() as u32,
            "ten".len() as u32,
            "eleven".len() as u32,
            "twelve".len() as u32,
            "thirteen".len() as u32,
            "fourteen".len() as u32,
            "fifteen".len() as u32,
            "sixteen".len() as u32,
            "seventeen".len() as u32,
            "eighteen".len() as u32,
            "nineteen".len() as u32,
        ];
        let n_10s: [u32; 10] = [
            0, // 0
            0, // 10
            "twenty".len() as u32,
            "thirty".len() as u32,
            "forty".len() as u32,
            "fifty".len() as u32,
            "sixty".len() as u32,
            "seventy".len() as u32,
            "eighty".len() as u32,
            "ninety".len() as u32,
        ];

        let mut tot: u32 = 0;

        if n >= 1000 {
            tot += n_0_19[n as usize / 1000] + "thousand".len() as u32;
            n %= 1000;
        }

        if n >= 100 {
            tot += n_0_19[n as usize / 100] + "hundred".len() as u32;

            if n % 100 != 0 {
                tot += "and".len() as u32;
            }

            n %= 100;
        }

        if n >= 20 {
            tot += n_10s[n as usize / 10];
            n %= 10;
        }

        tot += n_0_19[n as usize];

        tot
    }
}
