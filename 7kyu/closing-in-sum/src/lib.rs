//! <https://www.codewars.com/kata/65126d52a5de2b11c94096d2/train/rust>

pub const fn closing_in_sum(mut n: u64) -> u32 {
    let mut ones = 0;
    let mut decimal = 1;
    while n > decimal {
        ones += n % 10;
        n /= 10;
        decimal *= 10;
    }

    let mut tens = 0;
    while n != 0 {
        tens += n % 10;
        n /= 10;
    }

    ((tens * 10) + ones) as _
}
