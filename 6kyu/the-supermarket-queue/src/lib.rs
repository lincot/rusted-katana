//! <https://www.codewars.com/kata/57b06f90e298a7b53d000a86/train/rust>

use core::{cmp::Reverse, mem::transmute};
use std::collections::BinaryHeap;

pub fn queue_time(customers: &[u32], n: u32) -> u32 {
    assert!(n > 0 || customers.is_empty());
    if n as usize >= customers.len() {
        return *customers.iter().max().unwrap_or(&0);
    }
    if n == 1 {
        return customers.iter().sum();
    }
    match n {
        2 => queue_time_arr::<2>(customers),
        3 => queue_time_arr::<3>(customers),
        4 => queue_time_arr::<4>(customers),
        5 => queue_time_arr::<5>(customers),
        6 => queue_time_arr::<6>(customers),
        7 => queue_time_arr::<7>(customers),
        8 => queue_time_arr::<8>(customers),
        9 => queue_time_arr::<9>(customers),
        _ => queue_time_heap(customers, n),
    }
}

fn queue_time_arr<const N: usize>(customers: &[u32]) -> u32 {
    let mut q = [0; N];
    for t in customers {
        *q.iter_mut().min().unwrap() += t;
    }
    q.iter().max().copied().unwrap_or(0)
}

fn queue_time_heap(customers: &[u32], n: u32) -> u32 {
    let mut q: BinaryHeap<Reverse<u32>> = unsafe { transmute(vec![Reverse(0u32); n as _]) };
    for t in customers {
        let mut min = unsafe { q.peek_mut().unwrap_unchecked() };
        *min = Reverse(min.0 + t);
    }
    q.into_iter().map(|Reverse(t)| t).max().unwrap_or(0)
}
