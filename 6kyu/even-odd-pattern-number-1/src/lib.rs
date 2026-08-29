//! <https://www.codewars.com/kata/559e708e72d342b0c900007b/train/rust>

pub fn even_odd(arr: &[i64]) -> i64 {
    arr.iter()
        .enumerate()
        .fold(0, |acc, (i, x)| if i % 2 == 0 { acc + x } else { acc * x })
}
