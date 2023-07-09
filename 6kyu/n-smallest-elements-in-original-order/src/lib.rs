//! <https://www.codewars.com/kata/5aec1ed7de4c7f3517000079/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::mem::{forget, size_of};

const fn gcd(mut m: usize, mut n: usize) -> usize {
    if m == 0 || n == 0 {
        return m | n;
    }
    let shift = (m | n).trailing_zeros();
    m >>= m.trailing_zeros();
    n >>= n.trailing_zeros();
    while m != n {
        if m > n {
            m -= n;
            m >>= m.trailing_zeros();
        } else {
            n -= m;
            n >>= n.trailing_zeros();
        }
    }
    m << shift
}

pub fn first_n_smallest(arr: &[i32], n: usize) -> Vec<i32> {
    // 1 for 32/64/128 bit
    const N: usize = size_of::<i32>() / gcd(size_of::<i32>(), size_of::<(i32, usize)>());
    #[allow(clippy::modulo_one)]
    let mut arr_enumerated = Vec::with_capacity(
        arr.len()
            + if arr.len() % N == 0 {
                0
            } else {
                N - arr.len() % N
            },
    );
    unsafe { arr_enumerated.set_len(arr.len()) };
    let mut ptr = arr_enumerated.as_mut_ptr();
    for (i, &x) in arr.iter().enumerate() {
        unsafe {
            *ptr = (x, i);
            ptr = ptr.add(1);
        }
    }
    arr_enumerated.select_nth_unstable(n.saturating_sub(1));
    if n < 64 {
        unsafe { arr_enumerated.get_unchecked_mut(..n) }.sort_by_key(|&(_, i)| i);
    } else {
        radsort::sort_by_key(
            unsafe { arr_enumerated.get_unchecked_mut(..n) },
            |&(_, i)| i,
        );
    }
    let mut res_ptr = arr_enumerated.as_mut_ptr().cast();
    let mut ptr = arr_enumerated.as_mut_ptr();
    for _ in 0..n {
        unsafe {
            *res_ptr = (*ptr).0;
            res_ptr = res_ptr.add(1);
            ptr = ptr.add(1);
        }
    }
    let res = unsafe {
        Vec::from_raw_parts(
            arr_enumerated.as_mut_ptr().cast(),
            n,
            arr_enumerated.capacity() * size_of::<(i32, usize)>() / size_of::<i32>(),
        )
    };
    forget(arr_enumerated);
    res
}
