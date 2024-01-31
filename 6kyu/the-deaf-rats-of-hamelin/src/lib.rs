//! <https://www.codewars.com/kata/598106cb34e205e074000031/train/rust>

pub const fn count_deaf_rats(town: &str) -> u8 {
    let town = town.as_bytes();
    let mut i = 0;
    let mut res = 0;
    while i < town.len() {
        match town[i] {
            b'~' => {
                i += 1;
            }
            b'O' => {
                i += 1;
                res += 1;
            }
            b'P' => break,
            _ => {}
        }
        i += 1;
    }
    while i < town.len() {
        match town[i] {
            b'O' => {
                i += 1;
            }
            b'~' => {
                i += 1;
                res += 1;
            }
            _ => {}
        }
        i += 1;
    }
    res
}
