//! <https://www.codewars.com/kata/5720a1cb65a504fdff0003e2/train/rust>

pub fn difference_in_ages(ages: &[u8]) -> (u8, u8, u8) {
    let (min, max) = min_max(ages);
    (min, max, max - min)
}

fn min_max(arr: &[u8]) -> (u8, u8) {
    arr.iter().fold((u8::MAX, u8::MIN), |(min, max), &new| {
        (min.min(new), max.max(new))
    })
}
