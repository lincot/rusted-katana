//! <https://www.codewars.com/kata/545a4c5a61aa4c6916000755/train/rust>

pub const fn gimme(input_array: [i32; 3]) -> usize {
    let [a, b, c] = input_array;

    #[allow(clippy::match_same_arms)]
    match (a < b, b < c, c < a) {
        (false, _, false) => 0,
        (true, _, true) => 0,
        (true, true, _) => 1,
        (false, false, _) => 1,
        _ => 2,
    }
}
