#![no_std]
#![feature(test)]

extern crate alloc;
extern crate test;
use alloc::vec;
use parts_of_a_list::part_list;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let arr = black_box(vec!["I", "wish", "I", "hadn't", "come"]);
    bencher.iter(|| part_list(arr.clone()));
}
