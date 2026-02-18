//! <https://www.codewars.com/kata/55905b7597175ffc1a00005a/train/rust>

use digital::CountDigitsBase10;

pub fn page_digits(pages: u64) -> u64 {
    if pages < 10 {
        return pages;
    }
    let log10 = pages.count_digits_base10() as u64 - 1;
    let mut t = 1;
    for _ in 0..log10 {
        t = 10 * t + 1;
    }
    (log10 + 1) * pages + (log10 + 1) - t
}
