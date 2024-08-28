//! <https://www.codewars.com/kata/5cc70653658d6f002ab170b5/train/rust>

use digital::Next2Digits;

pub struct S {
    pub s: String,
    pub xs: Vec<i32>,
}

pub fn sqr_modulus(a: S) -> (bool, i32, i32) {
    match a.s.as_str() {
        "cart" => {
            let sum = a.xs.iter().map(|x| (x * x) as u32).sum();
            (true, sum as i32, descending_order(sum))
        }
        "polar" => {
            let sum = a.xs.chunks(2).map(|pair| (pair[0] * pair[0]) as u32).sum();
            (true, sum as i32, descending_order(sum))
        }
        _ => (false, -1, 1),
    }
}

fn descending_order(mut x: u32) -> i32 {
    let mut digit_counts = [0u8; 10];
    while let Some(digits) = x.next_2_digits() {
        for d in digits {
            digit_counts[d as usize] += 1;
        }
    }
    if x != 0 {
        digit_counts[x as usize] += 1;
    }

    let mut res = 0;
    for (i, &n) in digit_counts.iter().enumerate().rev() {
        let (a, b) = unsafe { TABLE.get_unchecked(n as usize) };
        res = res * a + b * i as i32;
    }
    res
}

const TABLE: [(i32, i32); 11] = [
    (1, 0),
    (10, 1),
    (100, 11),
    (1_000, 111),
    (10_000, 1_111),
    (100_000, 11_111),
    (1_000_000, 111_111),
    (10_000_000, 1_111_111),
    (100_000_000, 11_111_111),
    (1_000_000_000, 111_111_111),
    (1_410_065_408, 1_111_111_111),
];
