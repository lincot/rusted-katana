//! <https://www.codewars.com/kata/5676ffaa8da527f234000025/train/rust>

use digital::{MaxLenBase10, WriteNumUnchecked};

pub fn sc_perm_comb(num: u32) -> u64 {
    const ONES: [u64; 10] = {
        let mut res = [1; 10];
        let mut i = 1;
        while i < res.len() {
            res[i] = 10 * res[i - 1] + 1;
            i += 1;
        }
        res
    };

    const POWERS_OF_10: [u64; 10] = {
        let mut res = [1; 10];
        let mut i = 2;
        while i < res.len() {
            res[i] = 10 * res[i - 1];
            i += 1;
        }
        res
    };

    let mut digits = heapless::Vec::<usize, { u32::MAX_LEN_BASE10 }>::new();
    unsafe { digits.write_num_unchecked(num, 10, true, true) };
    if digits.is_empty() || digits.len() >= u32::MAX_LEN_BASE10 {
        unsafe { core::hint::unreachable_unchecked() };
    }
    digits.sort_unstable();

    if let Some(first_nonzero) = digits.iter().position(|&x| x != 0) {
        digits.swap(0, first_nonzero);
    }

    let mut res = 0;
    let mut min_len = 1;
    loop {
        res += digits
            .iter()
            .enumerate()
            .map(|(i, &d)| {
                d as u64
                    * unsafe {
                        if i < min_len {
                            ONES.get_unchecked(digits.len() - min_len)
                                * POWERS_OF_10.get_unchecked(min_len - i)
                        } else {
                            *ONES.get_unchecked(digits.len() - 1 - i)
                        }
                    }
            })
            .sum::<u64>();

        if let Some(i) = next_permutation(&mut digits) {
            min_len = i;
        } else {
            break;
        }
    }

    res
}

fn next_permutation(arr: &mut [usize]) -> Option<usize> {
    let mut i = arr.len() - 1;
    while i > 0 && arr[i - 1] >= arr[i] {
        i -= 1;
    }

    if i == 0 {
        return None;
    }

    let mut j = arr.len() - 1;
    while *unsafe { arr.get_unchecked(j) } <= arr[i - 1] {
        j -= 1;
    }

    arr.swap(i - 1, j);

    arr[i..].reverse();

    Some(i)
}
