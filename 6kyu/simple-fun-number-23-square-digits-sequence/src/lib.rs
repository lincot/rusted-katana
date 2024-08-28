//! <https://www.codewars.com/kata/5886d65e427c27afeb0000c1/train/rust>

use core::hint::unreachable_unchecked;

pub fn square_digits_sequence(a0: u32) -> usize {
    assert!(a0 <= 650);

    let mut an = a0;
    let mut seen = heapless::FnvIndexSet::<_, 32>::new();

    loop {
        if seen.len() == seen.capacity() {
            unsafe { unreachable_unchecked() };
        }
        unsafe { seen.insert(an).unwrap_unchecked() };

        let a_next = sum_squared_digits(an);

        if seen.contains(&a_next) {
            break;
        }
        an = a_next;
    }

    seen.len() + 1
}

#[inline]
const fn sum_squared_digits(mut x: u32) -> u32 {
    const SQUARED_DIGIT_SUMS_100: [u8; 100] = [
        0, 1, 4, 9, 16, 25, 36, 49, 64, 81, 1, 2, 5, 10, 17, 26, 37, 50, 65, 82, 4, 5, 8, 13, 20,
        29, 40, 53, 68, 85, 9, 10, 13, 18, 25, 34, 45, 58, 73, 90, 16, 17, 20, 25, 32, 41, 52, 65,
        80, 97, 25, 26, 29, 34, 41, 50, 61, 74, 89, 106, 36, 37, 40, 45, 52, 61, 72, 85, 100, 117,
        49, 50, 53, 58, 65, 74, 85, 98, 113, 130, 64, 65, 68, 73, 80, 89, 100, 113, 128, 145, 81,
        82, 85, 90, 97, 106, 117, 130, 145, 162,
    ];

    let mut res = 0;
    while x >= 10 {
        res += SQUARED_DIGIT_SUMS_100[(x % 100) as usize] as u32;
        x /= 100;
    }
    res += SQUARED_DIGIT_SUMS_100[x as usize] as u32;
    res
}
