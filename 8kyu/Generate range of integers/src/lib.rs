//! <https://www.codewars.com/kata/55eca815d0d20962e1000106/train/rust>

pub fn generate_range(min: usize, max: usize, step: usize) -> Vec<usize> {
    (min..=max).step_by(step).collect()
}
