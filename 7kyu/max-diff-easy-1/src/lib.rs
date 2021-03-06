//! <https://www.codewars.com/kata/588a3c3ef0fbc9c8e1000095/train/rust>

pub fn max_diff(numbers: &[i32]) -> i32 {
    let mut it = numbers.iter();

    let first = if let Some(&x) = it.next() {
        x
    } else {
        return 0;
    };

    let (min, max) = it.fold((first, first), |(min, max), &new| {
        (min.min(new), max.max(new))
    });

    max - min
}
