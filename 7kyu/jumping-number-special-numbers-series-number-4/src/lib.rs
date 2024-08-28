//! <https://www.codewars.com/kata/5a54e796b3bfa8932c0000ed/train/rust>

use digital::Next2Digits;

pub fn jumping_number(mut n: u64) -> String {
    let mut prev = (n % 10) as u8;
    n /= 10;

    while let Some([b, a]) = n.next_2_digits() {
        if (a != prev + 1 && a + 1 != prev) || (a != b + 1 && a + 1 != b) {
            return "Not!!".into();
        }

        prev = b;
    }

    if n == 0 || n as u8 == prev + 1 || n as u8 + 1 == prev {
        "Jumping!!".into()
    } else {
        "Not!!".into()
    }
}
