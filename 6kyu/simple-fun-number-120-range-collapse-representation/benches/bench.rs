#![feature(test)]

extern crate test;
use simple_fun_number_120_range_collapse_representation::descriptions;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| descriptions(black_box(&[3, 4, 5, 8, 9, 10, 11, 23, 43, 66, 67])));
}
