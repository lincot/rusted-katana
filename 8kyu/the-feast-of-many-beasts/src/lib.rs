//! <https://www.codewars.com/kata/5aa736a455f906981800360d/train/rust>

pub fn feast(beast: &str, dish: &str) -> bool {
    beast.chars().next().unwrap() == dish.chars().next().unwrap()
        && beast.chars().last().unwrap() == dish.chars().last().unwrap()
}
