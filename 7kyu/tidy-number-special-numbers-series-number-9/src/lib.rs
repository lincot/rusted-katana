//! <https://www.codewars.com/kata/5a87449ab1710171300000fd/train/rust>

use digital::Next2Digits;

pub fn tidy_number(mut n: u64) -> bool {
    let mut prev = 10;

    while let Some([b, a]) = n.next_2_digits() {
        if prev < a || a < b {
            return false;
        }

        prev = b;
    }

    n == 0 || prev >= n as u8
}
