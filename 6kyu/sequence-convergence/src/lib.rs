//! <https://www.codewars.com/kata/59971e64bfccc70748000068/train/rust>

pub const fn convergence(n: u32) -> usize {
    let mut seq_1 = 1;
    let mut seq_n = n;
    let mut res = 0;
    loop {
        while seq_1 > seq_n {
            seq_n += digits_product(seq_n);
            res += 1;
        }
        if seq_1 == seq_n {
            break;
        }
        seq_1 += digits_product(seq_1);
    }
    res
}

#[inline]
const fn digits_product(mut n: u32) -> u32 {
    const DIGIT_PRODUCTS_NONZERO_100: [u8; 100] = [
        1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 2, 2, 4, 6, 8, 10, 12, 14, 16,
        18, 3, 3, 6, 9, 12, 15, 18, 21, 24, 27, 4, 4, 8, 12, 16, 20, 24, 28, 32, 36, 5, 5, 10, 15,
        20, 25, 30, 35, 40, 45, 6, 6, 12, 18, 24, 30, 36, 42, 48, 54, 7, 7, 14, 21, 28, 35, 42, 49,
        56, 63, 8, 8, 16, 24, 32, 40, 48, 56, 64, 72, 9, 9, 18, 27, 36, 45, 54, 63, 72, 81,
    ];

    let mut res = 1;
    while n >= 10 {
        res *= DIGIT_PRODUCTS_NONZERO_100[(n % 100) as usize] as u32;
        n /= 100;
    }
    if n != 0 {
        res *= n;
    }
    res
}
