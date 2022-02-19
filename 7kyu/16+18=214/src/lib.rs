//! <https://www.codewars.com/kata/5effa412233ac3002a9e471d/train/rust>

pub const fn add(mut num1: u32, mut num2: u32) -> u64 {
    let mut res = 0;
    let mut m = 1;

    while num1 != 0 || num2 != 0 {
        let s = num1 % 10 + num2 % 10;
        res += s as u64 * m;
        m *= if s < 10 { 10 } else { 100 };

        num1 /= 10;
        num2 /= 10;
    }

    res
}
