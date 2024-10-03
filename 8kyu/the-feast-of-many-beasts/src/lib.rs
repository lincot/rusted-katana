//! <https://www.codewars.com/kata/5aa736a455f906981800360d/train/rust>

pub fn feast(beast: &str, dish: &str) -> bool {
    beast.chars().next() == dish.chars().next() && beast.chars().last() == dish.chars().last()
}
