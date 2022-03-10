#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let vec = black_box(&[
        22, 23, 3, 16, 10, 9, 28, 24, 20, 4, 9, 16, 20, 26, 15, 22, 24, 10, 17, 11, 30, 1, 5, 5, 0,
        3, 25, 23, 26, 13, 19, 20, 13, 21, 14, 7, 12, 26, 19, 26, 19, 13, 9, 16, 7, 16, 14, 0, 26,
        22, 19, 25, 28, 7, 10, 11, 9, 21, 14, 21, 17, 17, 17, 3, 26, 11, 8, 23, 2, 1, 1, 4, 27, 28,
        6, 19, 18, 24, 13, 14, 30, 21, 10, 18, 28, 19, 29, 21, 13, 9, 18, 28, 22, 17, 7, 29, 29,
        17, 29, 0, 21, 27, 23, 0, 24, 5, 3, 8, 10, 14, 26, 1, 21, 26, 0, 6, 28, 30, 9, 10, 15, 26,
        15, 9, 29, 6, 19, 18, 28, 11, 17, 19, 30, 19, 8, 19, 27, 13, 30, 17, 11, 6, 5, 1, 0, 24,
        20, 13, 24, 25, 5, 28, 16, 11, 19, 21, 11, 13, 28, 20, 5, 6, 30, 8, 26, 6, 17, 15, 11, 9,
        16, 15, 15, 3, 21, 16, 14, 24, 2, 28, 17, 24, 19, 12, 8, 3, 18, 27, 25, 24, 4, 27, 0, 28,
        1, 25, 4, 15, 8, 27, 13, 2, 30, 3, 15, 5, 10, 30, 27, 27, 30, 6, 14, 20, 25, 18, 1, 24, 1,
        27, 2, 8, 4, 7, 27, 26, 27, 12, 5, 17, 11, 20, 20, 22, 9, 18, 1, 27, 23, 11, 19, 16, 6, 6,
        24, 28, 19, 17, 14, 24,
    ]);
    bencher.iter(|| solution::solve(vec))
}
