//! <https://www.codewars.com/kata/59b401e24f98a813f9000026/train/rust>

use digital::Next2Digits;

pub fn compute_depth(n: u16) -> u8 {
    let mut n_multiple = n;
    let mut found_digits = 0;
    let mut res = 0;

    while found_digits != (1 << 10) - 1 {
        let mut m = n_multiple;

        while let Some([a, b]) = m.next_2_digits() {
            found_digits |= 1 << a;
            found_digits |= 1 << b;
        }
        if m != 0 {
            found_digits |= 1 << m;
        }

        n_multiple += n;
        res += 1;
    }

    res
}
