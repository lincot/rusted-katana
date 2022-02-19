//! <https://www.codewars.com/kata/554b4ac871d6813a03000035/train/rust>

pub fn high_and_low(numbers: &str) -> String {
    let mut min = i32::MAX;
    let mut max = i32::MIN;

    for n in numbers.split_ascii_whitespace().map(|s| s.parse().unwrap()) {
        min = min.min(n);
        max = max.max(n);
    }

    format!("{} {}", max, min)
}
