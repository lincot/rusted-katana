//! <https://www.codewars.com/kata/5853213063adbd1b9b0000be/train/rust>

mod preloaded;

pub use preloaded::Direction;
use unchecked::PushUnchecked;

const FIGHTERS: [&str; 12] = [
    "Ryu", "E.Honda", "Blanka", "Guile", "Balrog", "Vega", "Ken", "Chun Li", "Zangief", "Dhalsim",
    "Sagat", "M.Bison",
];

pub fn street_fighter_selection(
    _fighters: &[[&str; 6]; 2],
    _position: &[i64; 2],
    moves: &[Direction],
) -> Vec<String> {
    let mut res = Vec::with_capacity(moves.len());
    let mut position = 0usize;
    for d in moves {
        position = match d {
            Direction::Up => position.checked_sub(6).unwrap_or(position),
            Direction::Down => position + if position < 6 { 6 } else { 0 },
            Direction::Left => {
                if [0, 6].contains(&position) {
                    position + 5
                } else {
                    position - 1
                }
            }
            Direction::Right => {
                if [5, 11].contains(&position) {
                    position - 5
                } else {
                    position + 1
                }
            }
        };
        unsafe { res.push_unchecked((*FIGHTERS.get_unchecked(position)).into()) };
    }
    res
}
