#![no_std]
#![feature(test)]

extern crate test;
use peak_array_index::peak;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let arr = black_box(&[
        165, 238, 120, 186, 174, 48, 19, 194, 77, 58, 198, 72, 198, 121, 61, 103, 149, 37, 188,
        141, 47, 206, 195, 79, 82, 186, 67, 238, 129, 32, 53, 237, 250, 211, 36, 135, 227, 64, 189,
        253, 5, 173, 167, 3, 123, 98, 43, 230, 185, 163, 252, 94, 170, 88, 119, 168, 64, 134, 81,
    ]);
    bencher.iter(|| peak(arr));
}
