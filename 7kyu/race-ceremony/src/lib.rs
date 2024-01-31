//! <https://www.codewars.com/kata/62cecd4e5487c10028996e04/train/rust>

pub const fn race_podium(blocks: u32) -> (u32, u32, u32) {
    let q = blocks / 3;
    let r = blocks % 3;
    (
        q + (r != 0) as u32 - (blocks == 7) as u32,
        q + (r != 0) as u32 + 1,
        q - 1 - (r == 1 && blocks != 7) as u32,
    )
}
