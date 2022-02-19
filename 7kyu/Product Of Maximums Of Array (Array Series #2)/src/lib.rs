//! <https://www.codewars.com/kata/5a63948acadebff56f000018/train/rust>

#![feature(binary_heap_into_iter_sorted)]

use std::collections::BinaryHeap;

pub fn max_product(lst: Vec<i32>, n_largest_elements: i32) -> i32 {
    let n_largest_elements = n_largest_elements as usize;

    if n_largest_elements < lst.len() / 2 {
        max_product_heap(lst, n_largest_elements)
    } else {
        max_product_sort(lst, n_largest_elements)
    }
}

fn max_product_heap(lst: Vec<i32>, n_largest_elements: usize) -> i32 {
    BinaryHeap::from(lst)
        .into_iter_sorted()
        .take(n_largest_elements)
        .product()
}

fn max_product_sort(mut lst: Vec<i32>, n_largest_elements: usize) -> i32 {
    lst.sort_unstable();
    lst.iter().rev().take(n_largest_elements).product()
}
