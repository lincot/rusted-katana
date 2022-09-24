//! <https://www.codewars.com/kata/5672a98bdbdd995fad00000f/train/rust>

#![no_std]

pub const fn rps(p1: &str, p2: &str) -> &'static str {
    match &[p1.as_bytes()[0], p2.as_bytes()[0]] {
        b"rs" | b"sp" | b"pr" => "Player 1 won!",
        b"sr" | b"ps" | b"rp" => "Player 2 won!",
        _ => "Draw!",
    }
}
