#![no_std]
#![feature(test)]

extern crate alloc;
extern crate test;
use alloc::vec;
use numbers_to_letters::switcher;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let numbers = black_box(vec![
        "25", "7", "8", "4", "14", "23", "8", "25", "23", "29", "16", "16", "4",
    ]);
    bencher.iter(|| switcher(numbers.clone()));
}
