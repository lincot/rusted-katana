//! <https://www.codewars.com/kata/55eeddff3f64c954c2000059/train/rust>

pub type Number = i32;
pub type Numbers = Vec<Number>;

pub fn sum_consecutives(numbers: &[i32]) -> Vec<i32> {
    let mut res = Vec::with_capacity(numbers.len());

    let mut numbers = numbers.iter();

    let mut prev_number = if let Some(first) = numbers.next() {
        first
    } else {
        return res;
    };
    let mut same_count = 1;

    for n in numbers {
        if n != prev_number {
            res.push(same_count * prev_number);
            prev_number = n;
            same_count = 1;
        } else {
            same_count += 1;
        }
    }
    res.push(same_count * prev_number);

    res
}
