//! <https://www.codewars.com/kata/5bb148b840196d1be50000b1/train/rust>

use char_to_lower::to_lower;

pub fn convert(word: &str) -> u64 {
    let mut res = 0;
    let mut map = heapless::Vec::<_, 10>::new();
    for c in word.chars().map(to_lower) {
        let len = map.len() as u8;
        let digit = if let Some(&(_, count)) = map.iter().find(|&&(x, _)| x == c) {
            count
        } else {
            map.push((c, len)).unwrap();
            len
        };

        let digit = if digit == 0 {
            1
        } else if digit == 1 {
            0
        } else {
            digit
        };
        res *= 10;
        res += digit as u64;
    }
    res
}
