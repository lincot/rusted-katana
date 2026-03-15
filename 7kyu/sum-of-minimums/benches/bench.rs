#![feature(test)]

extern crate test;
use sum_of_minimums::sum_of_minimums;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        sum_of_minimums(black_box([
            [7, 9, 8, 6],
            [6, 5, 4, 3],
            [5, 7, 4, 5],
            [7, 9, 4, 3],
        ]))
    });
}
