//! <https://www.codewars.com/kata/545a4c5a61aa4c6916000755/train/rust>

pub const fn gimme(input_array: [i32; 3]) -> usize {
    let [a, b, c] = input_array;

    match (a < b, b < c, c < a) {
        (false, _, false) | (true, _, true) => 0,
        (true, true, _) | (false, false, _) => 1,
        _ => 2,
    }
}
