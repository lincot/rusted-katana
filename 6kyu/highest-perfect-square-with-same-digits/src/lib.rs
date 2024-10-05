//! <https://www.codewars.com/kata/5b2cd515553292a4ff0000c2/train/rust>

use core::cmp::Reverse;
use digital::{MaxLenBase10, WriteNumUnchecked};

pub fn next_perfectsq_perm(lower_limit: u32, k: u32) -> u32 {
    const POWERS_OF_10: [u32; 10] = {
        let mut res = [1; 10];
        let mut i = 1;
        while i < res.len() {
            res[i] = 10 * res[i - 1];
            i += 1;
        }
        res
    };

    const ONES: [u32; 9] = {
        let mut res = [1; 9];
        let mut i = 1;
        while i < res.len() {
            res[i] = 10 * res[i - 1] + 1;
            i += 1;
        }
        res
    };

    let mut n = lower_limit + 1;
    loop {
        let mut digits = heapless::Vec::<usize, { u32::MAX_LEN_BASE10 }>::new();
        unsafe { digits.write_num_unchecked(n, 10, true, true) };
        if digits.is_empty() || digits.len() >= u32::MAX_LEN_BASE10 {
            unsafe { core::hint::unreachable_unchecked() };
        }

        for i in (0..digits.len() - 1).rev() {
            if digits[i] == 0 {
                for j in 0..i + 1 {
                    digits[j] = 1;
                }
                n += ONES[i] - n % POWERS_OF_10[i + 1];
                break;
            }
        }

        if is_square(n) {
            digits.sort_unstable_by_key(|&x| Reverse(x));
            let mut num_squares = 1;
            let mut max_square = n;
            loop {
                let m = digits
                    .iter()
                    .enumerate()
                    .map(|(i, &d)| d as u32 * unsafe { POWERS_OF_10.get_unchecked(i) })
                    .sum();
                if m != n && is_square(m) {
                    num_squares += 1;
                    if num_squares > k {
                        break;
                    }
                    max_square = m;
                }

                if !next_permutation(&mut digits) {
                    break;
                }
            }

            if num_squares == k {
                return max_square.max(n);
            }
        }

        n += 1;
    }
}

fn is_square(n: u32) -> bool {
    [0, 1, 4, 9].contains(&(n & 0xf)) && {
        let s: u32 = unsafe { (n as f64).sqrt().to_int_unchecked() };
        s * s == n
    }
}

fn next_permutation(arr: &mut [usize]) -> bool {
    let mut i = 0;
    while i + 1 < arr.len() && arr[i] <= arr[i + 1] {
        i += 1;
    }

    if i + 1 >= arr.len() {
        return false;
    }

    let mut j = 0;
    while *unsafe { arr.get_unchecked(j) } <= arr[i + 1] {
        j += 1;
    }

    arr.swap(i + 1, j);

    arr[..=i].reverse();

    true
}
