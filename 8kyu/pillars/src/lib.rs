//! <https://www.codewars.com/kata/5bb0c58f484fcd170700063d/train/rust>

pub const fn pillars(num_of_pillars: u32, distance: u32, width: u32) -> u32 {
    if num_of_pillars < 2 {
        0
    } else {
        let distance = 100 * distance;
        (distance + width) * (num_of_pillars - 2) + distance
    }
}
