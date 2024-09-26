//! <https://www.codewars.com/kata/5a1d86dbba2a142e040000ee/train/rust>

use digital::Next2Digits;

pub fn find_number(a: u32, b: u32, s: &str) -> Vec<u32> {
    let [mut digits_ab, mut digits_s] = [[0usize; 10]; 2];
    for mut n in a..=b {
        if n == 0 {
            digits_ab[0] += 1;
        }
        while let Some(digits) = n.next_2_digits(true) {
            for d in digits {
                digits_ab[d as usize] += 1;
            }
        }
        if n != 0 {
            digits_ab[n as usize] += 1;
        }
    }

    for b in s.as_bytes() {
        digits_s[(b - b'0') as usize] += 1;
    }

    let mut res = Vec::new();

    let mut digits = Vec::new();
    for i in 0..10 {
        if digits_ab[i] < digits_s[i] {
            return res;
        }
        for _ in 0..digits_ab[i] - digits_s[i] {
            digits.push(i as _);
        }
    }

    if digits.is_empty() {
        return res;
    }

    loop {
        if digits[0] != 0 {
            let num = digits.iter().fold(0, |acc, &d| 10 * acc + d as u32);
            if (a..=b).contains(&num) {
                res.push(num);
            }
        }

        if !next_permutation(&mut digits) {
            break;
        }
    }
    res
}

fn next_permutation(arr: &mut [u8]) -> bool {
    let mut i = arr.len() - 1;
    while i > 0 && arr[i - 1] >= arr[i] {
        i -= 1;
    }

    if i == 0 {
        return false;
    }

    let mut j = arr.len() - 1;
    while *unsafe { arr.get_unchecked(j) } <= arr[i - 1] {
        j -= 1;
    }

    arr.swap(i - 1, j);

    arr[i..].reverse();

    true
}
