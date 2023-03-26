//! <https://www.codewars.com/kata/64087fd72daf09000f60dc26/train/rust>

#![no_std]

pub fn cute_pattern(tiles: &str) -> bool {
    let tiles = tiles.as_bytes();
    if tiles.len() < 5 * 4 - 1 {
        return false;
    }
    for row in 0..3 {
        for column in 0..3 {
            let start = 5 * row + column;
            if [1, 5, 5 + 1]
                .into_iter()
                .all(|x| tiles[start + x] == tiles[start])
            {
                return false;
            }
        }
    }
    true
}
