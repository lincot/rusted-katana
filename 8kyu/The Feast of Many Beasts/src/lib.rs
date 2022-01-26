//! <https://www.codewars.com/kata/5aa736a455f906981800360d/train/rust>

pub fn feast(beast: &str, dish: &str) -> bool {
    let mut beast = beast.chars();
    let mut dish = dish.chars();

    beast.next() == dish.next() && beast.last() == dish.last()
}
