//! <https://www.codewars.com/kata/5bb3e299484fcd5dbb002912/train/rust>

pub fn pyramid(balls: u16) -> u16 {
    (unsafe {
        (balls as f64)
            .mul_add(8., 1.)
            .sqrt()
            .to_int_unchecked::<u16>()
    } - 1)
        / 2
}
