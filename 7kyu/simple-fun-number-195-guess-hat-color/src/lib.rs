//! <https://www.codewars.com/kata/58c21c4ff130b7cab400009e/train/rust>

mod preloaded;

use preloaded::Color;

pub fn guess_hat_color(_a: Color, b: Color, c: Color, _d: Color) -> usize {
    if b == c {
        1
    } else {
        2
    }
}
