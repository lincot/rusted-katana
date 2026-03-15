//! <https://www.codewars.com/kata/5bb3e299484fcd5dbb002912/train/rust>

#[expect(clippy::suboptimal_flops)]
pub fn pyramid(balls: u16) -> u16 {
    (unsafe { (8. * balls as f64 + 1.).sqrt().to_int_unchecked::<u16>() } - 1) / 2
}
