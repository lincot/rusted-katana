#![feature(test)]

extern crate test;
use ascii_fun_number_1_x_shape::x;
use test::{Bencher, black_box};

#[bench]
fn bench_3(bencher: &mut Bencher) {
    bencher.iter(|| x(black_box(3)));
}

#[bench]
fn bench_11(bencher: &mut Bencher) {
    bencher.iter(|| x(black_box(11)));
}

#[bench]
fn bench_99(bencher: &mut Bencher) {
    bencher.iter(|| x(black_box(99)));
}
