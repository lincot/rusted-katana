//! <https://www.codewars.com/kata/558fc85d8fd1938afb000014/train/rust>

pub const fn sum_two_smallest_numbers(numbers: &[u32]) -> u32 {
    let [a, b, rest @ ..] = numbers else {
        return 0;
    };

    let mut min0 = if *a < *b { *a } else { *b };
    let mut min1 = if *a < *b { *b } else { *a };
    let mut i = 0;
    while i < rest.len() {
        let n = rest[i];
        if n <= min0 {
            min1 = min0;
            min0 = n;
        } else if n <= min1 {
            min1 = n;
        }
        i += 1;
    }
    min0 + min1
}
