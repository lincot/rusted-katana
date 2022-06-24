//! <https://www.codewars.com/kata/54fe05c4762e2e3047000add/train/rust>

pub struct Ship {
    draft: u32,
    crew: u32,
}
impl Ship {
    pub const fn is_worth_it(&self) -> bool {
        self.draft > 3 * self.crew / 2 + 20
    }
}
