//! <https://www.codewars.com/kata/5effa412233ac3002a9e471d/train/rust>

use digital::Next2Digits;

pub fn add(mut num1: u32, mut num2: u32) -> u64 {
    let mut res = 0;
    let mut m = 1;

    while num1 >= 10 || num2 >= 10 {
        let [b1, a1] = num1.next_2_digits().unwrap_or_else(|| {
            let t = num1;
            num1 = 0;
            [0, t as _]
        });
        let [b2, a2] = num2.next_2_digits().unwrap_or_else(|| {
            let t = num2;
            num2 = 0;
            [0, t as _]
        });

        let s = a1 as u64 + a2 as u64;
        res += s * m;
        m *= if s < 10 { 10 } else { 100 };

        let s = b1 as u64 + b2 as u64;
        res += s * m;
        m *= if s < 10 { 10 } else { 100 };
    }

    if num1 != 0 || num2 != 0 {
        let s = num1 + num2;
        res += s as u64 * m;
    }

    res
}
