//! <https://www.codewars.com/kata/5949481f86420f59480000e7/train/rust>

pub fn odd_or_even(numbers: Vec<i32>) -> String {
    if numbers.into_iter().sum::<i32>() % 2 == 0 {
        "even".into()
    } else {
        "odd".into()
    }
}
