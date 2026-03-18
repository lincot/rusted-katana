#![feature(test)]

extern crate test;
use get_the_mean_of_an_array::get_average;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        get_average(black_box(&[
            1, 2, 15, 15, 17, 11, 12, 17, 17, 14, 13, 15, 6, 11, 8, 7,
        ]))
    });
}
