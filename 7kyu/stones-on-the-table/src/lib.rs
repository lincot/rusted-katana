//! <https://www.codewars.com/kata/5f70e4cce10f9e0001c8995a/train/rust>

pub const fn stones_to_remove(stones: &str) -> usize {
    let stones = stones.as_bytes();
    if stones.len() <= 1 {
        return 0;
    }

    let mut i = 0;
    let mut res = 0;
    while i < stones.len() - 1 {
        if stones[i] == stones[i + 1] {
            res += 1;
        }
        i += 1;
    }
    res
}
