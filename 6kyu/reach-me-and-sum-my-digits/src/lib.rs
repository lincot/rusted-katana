//! <https://www.codewars.com/kata/55ffb44050558fdb200000a4/train/rust>

pub fn sum_dig_nth_term(init_val: u32, pattern: &[u32], nth: usize) -> u32 {
    if nth == 0 || pattern.is_empty() {
        return init_val;
    }

    let nth = nth - 1;
    let q = (nth / pattern.len()) as u32;
    let r = nth % pattern.len();

    sum_digits(
        init_val
            + (q + 1) * pattern[..r].iter().sum::<u32>()
            + q * pattern[r..].iter().sum::<u32>(),
    )
}

const fn sum_digits(mut num: u32) -> u32 {
    const DIGIT_SUMS_100: [u32; 100] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        11, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 5, 6, 7, 8, 9, 10,
        11, 12, 13, 14, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 8,
        9, 10, 11, 12, 13, 14, 15, 16, 17, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
    ];

    let mut res = 0;
    while num >= 100 {
        res += DIGIT_SUMS_100[(num % 100) as usize];
        num /= 100;
    }
    while num != 0 {
        res += num % 10;
        num /= 10;
    }
    res
}
