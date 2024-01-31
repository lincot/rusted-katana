//! <https://www.codewars.com/kata/5dae2599a8f7d90025d2f15f/train/rust>

pub const fn consecutive_ducks(n: u32) -> bool {
    !n.is_power_of_two()
}
