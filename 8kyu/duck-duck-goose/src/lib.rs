//! <https://www.codewars.com/kata/582e0e592029ea10530009ce/train/rust>

pub use preloaded::Player;

mod preloaded;

pub fn duck_duck_goose(players: &[Player], goose: u32) -> &'static str {
    players[(goose as usize - 1) % players.len()].name
}
