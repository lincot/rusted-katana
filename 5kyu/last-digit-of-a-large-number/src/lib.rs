//! <https://www.codewars.com/kata/5511b2f550906349a70004e1/train/rust>

const TABLE: [[u8; 4]; 10] = [
    [0, 0, 0, 0],
    [1, 1, 1, 1],
    [6, 2, 4, 8],
    [1, 3, 9, 7],
    [6, 4, 6, 4],
    [5, 5, 5, 5],
    [6, 6, 6, 6],
    [1, 7, 9, 3],
    [6, 8, 4, 2],
    [1, 9, 1, 9],
];

pub fn last_digit(str1: &str, str2: &str) -> i32 {
    if str2 == "0" {
        return 1;
    }

    let x_last = str1.as_bytes().last().unwrap() - b'0';
    let pow_last_2 = if str2.len() >= 2 {
        10 * (str2.as_bytes()[str2.len() - 2] - b'0')
    } else {
        0
    } + (str2.as_bytes().last().unwrap() - b'0');

    TABLE[x_last as usize][(pow_last_2 % 4) as usize] as _
}
