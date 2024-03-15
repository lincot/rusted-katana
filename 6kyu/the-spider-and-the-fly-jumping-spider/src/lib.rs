//! <https://www.codewars.com/kata/5a30e7e9c5e28454790000c1/train/rust>

use core::f64::consts::FRAC_1_SQRT_2;

pub fn spider_to_fly(spider: &str, fly: &str) -> f64 {
    const COS: [f64; 8] = [
        -1.,
        -FRAC_1_SQRT_2,
        0.,
        FRAC_1_SQRT_2,
        1.,
        FRAC_1_SQRT_2,
        0.,
        -FRAC_1_SQRT_2,
    ];

    let [a, b] =
        [spider, fly].map(|s| ((s.as_bytes()[1] - b'0') as usize, s.as_bytes()[0] as usize));

    ((2 * a.0 * b.0) as f64)
        .mul_add(COS[a.1.abs_diff(b.1)], (a.0.pow(2) + b.0.pow(2)) as f64)
        .sqrt()
}
