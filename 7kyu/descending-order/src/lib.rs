//! <https://www.codewars.com/kata/5467e4d82edf8bbf40000155/train/rust>

#![no_std]

const TABLE: [(u64, u64); 21] = [
    (1, 0),
    (10, 1),
    (100, 11),
    (1_000, 111),
    (10_000, 1_111),
    (100_000, 11_111),
    (1_000_000, 111_111),
    (10_000_000, 1_111_111),
    (100_000_000, 11_111_111),
    (1_000_000_000, 111_111_111),
    (10_000_000_000, 1_111_111_111),
    (100_000_000_000, 11_111_111_111),
    (1_000_000_000_000, 111_111_111_111),
    (10_000_000_000_000, 1_111_111_111_111),
    (100_000_000_000_000, 11_111_111_111_111),
    (1_000_000_000_000_000, 111_111_111_111_111),
    (10_000_000_000_000_000, 1_111_111_111_111_111),
    (100_000_000_000_000_000, 11_111_111_111_111_111),
    (1_000_000_000_000_000_000, 111_111_111_111_111_111),
    (10_000_000_000_000_000_000, 1_111_111_111_111_111_111),
    (7_766_279_631_452_241_920, 11_111_111_111_111_111_111),
];

pub fn descending_order(mut x: u64) -> u64 {
    let mut digits = [0u8; 10];

    while x != 0 {
        unsafe {
            *digits.get_unchecked_mut((x % 10) as usize) += 1;
        }
        x /= 10;
    }

    for (i, &n) in digits.iter().enumerate().rev() {
        let (a, b) = unsafe { TABLE.get_unchecked(n as usize) };
        x = x * a + b * i as u64;
    }

    x
}
