//! <https://www.codewars.com/kata/6135e4f40cffda0007ce356b/train/rust>

pub fn bishop(start_pos: &str, end_pos: &str, num_moves: u8) -> bool {
    assert!(start_pos.len() >= 2 && end_pos.len() >= 2);
    let (start, end) = (start_pos.as_bytes(), end_pos.as_bytes());
    let (diff0, diff1) = (end[0].abs_diff(start[0]), end[1].abs_diff(start[1]));
    start[0] == end[0] && start[1] == end[1]
        || num_moves >= 1 && diff0 == diff1
        || num_moves >= 2 && diff0 % 2 == diff1 % 2
}
