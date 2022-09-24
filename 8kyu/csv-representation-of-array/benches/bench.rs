#![no_std]
#![feature(test)]

extern crate alloc;
extern crate test;
use alloc::vec;
use csv_representation_of_array::to_csv_text;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let array = [
        vec![0, 1, 2, 3, 45, 0, 1, 2, 3, 45],
        vec![10, 11, 12, 13, 14, 10, 11, 12, 13, 14],
        vec![20, 21, 22, 23, 24, 20, 21, 22, 23, 24],
        vec![30, 31, 32, 33, 34, 30, 31, 32, 33, 34],
        vec![30, 31, 32, 33, 34, 30, 31, 32, 33, 34],
        vec![30, 31, 32, 33, 34, 30, 31, 32, 33, 34],
        vec![30, 31, 32, 33, 34, 30, 31, 32, 33, 34],
        vec![30, 31, 32, 33, 34, 30, 31, 32, 33, 34],
        vec![30, 31, 32, 33, 34, 30, 31, 32, 33, 34],
        vec![30, 31, 32, 33, 34, 30, 31, 32, 33, 34],
    ];
    let array = black_box(&array);
    bencher.iter(|| to_csv_text(array));
}
