//! <https://www.codewars.com/kata/59971e64bfccc70748000068/train/rust>

#![no_std]

const fn product_of_digits(mut n: u32) -> u32 {
    let mut res = 1;
    while n != 0 {
        let d = n % 10;
        if d != 0 {
            res *= d;
        }
        n /= 10;
    }
    res
}

pub const fn convergence(n: u32) -> usize {
    let mut seq_1 = 1;
    let mut seq_n = n;
    let mut res = 0;
    loop {
        while seq_1 > seq_n {
            seq_n += product_of_digits(seq_n);
            res += 1;
        }
        if seq_1 == seq_n {
            break;
        }
        seq_1 += product_of_digits(seq_1);
    }
    res
}
