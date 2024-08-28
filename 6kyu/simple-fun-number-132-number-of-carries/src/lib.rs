//! <https://www.codewars.com/kata/58a6568827f9546931000027/train/rust>

use digital::Next2Digits;

pub fn number_of_carries(mut a: u32, mut b: u32) -> usize {
    let mut res = 0;
    let mut carry = 0;
    while a >= 10 || b >= 10 {
        let [b1, a1] = a.next_2_digits().unwrap_or_else(|| {
            let t = a;
            a = 0;
            [0, t as _]
        });
        let [b2, a2] = b.next_2_digits().unwrap_or_else(|| {
            let t = b;
            b = 0;
            [0, t as _]
        });

        for [da, db] in [[a1 as u32, a2 as u32], [b1 as u32, b2 as u32]] {
            if da + db + carry >= 10 {
                res += 1;
                carry = 1;
            } else {
                carry = 0;
            }
        }
    }
    if a + b + carry >= 10 {
        res += 1;
    }
    res
}
