//! <https://www.codewars.com/kata/5ac6932b2f317b96980000ca/train/rust>

pub fn min_value(digits: Vec<i32>) -> i32 {
    let mut ds = [false; 10];
    for d in digits {
        if (1..=9).contains(&d) {
            ds[d as usize] = true;
        }
    }
    let mut res = 0;
    for (i, &d) in (0..).zip(&ds) {
        if d {
            res *= 10;
            res += i;
        }
    }
    res
}
