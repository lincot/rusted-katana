//! <https://www.codewars.com/kata/5a805d8cafa10f8b930005ba/train/rust>

pub fn nearest_sq(n: u32) -> u32 {
    let root: u32 = unsafe { (n as f64).sqrt().to_int_unchecked() };
    let square_below = root.pow(2);
    let square_above = (root + 1).pow(2);

    if n - square_below > square_above - n {
        square_above
    } else {
        square_below
    }
}
