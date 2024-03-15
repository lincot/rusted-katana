//! <https://www.codewars.com/kata/5f5daf1a209a64001183af9b/train/rust>

pub fn make_chocolates(small: u32, big: u32, goal: u32) -> Option<u32> {
    let mut big = big.min(goal / 5);
    if goal % 2 != big % 2 {
        big = big.saturating_sub(1);
    }
    let goal = goal - 5 * big;
    (goal % 2 == 0 && goal / 2 <= small).then_some(goal / 2)
}
