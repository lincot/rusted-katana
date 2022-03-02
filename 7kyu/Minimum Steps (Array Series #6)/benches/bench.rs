#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let nums = black_box(&[
        56, 21, 34, 96, 62, 47, 38, 35, 14, 97, 85, 97, 70, 59, 51, 42, 37, 34, 45, 22, 87, 91, 58,
        82, 78, 33, 44, 91, 27, 65, 14, 18, 22, 32, 65, 64, 45, 74, 23, 62, 6, 15, 57, 46, 67, 61,
        99, 33, 44, 4, 50, 64, 27, 7, 57, 85, 1, 83, 10, 6, 99, 72, 1, 60, 8, 26, 50, 70, 13, 79,
        34, 15, 57, 59, 45, 96, 32, 63, 14, 38, 49, 41, 57, 33, 44, 70, 16, 59, 56, 20, 70, 36, 72,
        18, 32, 89, 29, 47, 6, 37,
    ]);
    let value = black_box(1000);
    bencher.iter(|| solution::minimum_steps(nums, value))
}
