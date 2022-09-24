#![no_std]
#![feature(test)]

extern crate alloc;
extern crate test;
use alloc::vec;
use categorize_new_member::open_or_senior;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let data = black_box(vec![(45, 12), (55, 21), (19, -2), (104, 20)]);
    bencher.iter(|| open_or_senior(data.clone()));
}
