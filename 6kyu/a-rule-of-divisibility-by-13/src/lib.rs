//! <https://www.codewars.com/kata/564057bc348c7200bd0000ff/train/rust>

use digital::Next2Digits;

pub fn thirt(n: i64) -> i64 {
    let mut n = n as u64;
    while n >= 100 {
        n = lower_equiv_base13(n);
    }
    n as _
}

fn lower_equiv_base13(mut n: u64) -> u64 {
    const LOOP: [u64; 6] = [1, 10, 9, 12, 3, 4];

    let mut res = 0;
    let mut loop_i = 0;

    while let Some([a, b]) = n.next_2_digits(true) {
        res += unsafe {
            LOOP.get_unchecked(loop_i) * b as u64 + LOOP.get_unchecked(loop_i + 1) * a as u64
        };
        loop_i += 2;
        if loop_i >= LOOP.len() {
            loop_i = 0;
        }
    }

    if n != 0 {
        res += unsafe { LOOP.get_unchecked(loop_i) } * n;
    }
    res
}
