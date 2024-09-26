//! <https://www.codewars.com/kata/566fc12495810954b1000030/train/rust>

use digital::Next2Digits;

pub fn nb_dig(n: i32, d: i32) -> i32 {
    let n = n as u32;
    let d = d as u32;
    let mut res = (d == 0) as _;

    let mut num = 0;
    let mut step = 1;
    #[allow(clippy::range_plus_one)]
    for _ in 0..n + 1 {
        let mut m = num;
        while let Some([a, b]) = m.next_2_digits(true) {
            if a == d as u8 {
                res += 1;
            }
            if b == d as u8 {
                res += 1;
            }
        }

        if m != 0 && m == d {
            res += 1;
        }

        num += step;
        step += 2;
    }

    res
}
