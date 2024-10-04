#![feature(test)]

extern crate test;
use decimal_time_conversion::{string_to_industrial, to_industrial, to_normal};
use test::{black_box, Bencher};

#[bench]
fn bench_string_to_industrial(bencher: &mut Bencher) {
    bencher.iter(|| string_to_industrial(black_box("1:45")));
}

#[bench]
fn bench_to_industrial(bencher: &mut Bencher) {
    bencher.iter(|| to_industrial(black_box(105)));
}

#[bench]
fn bench_to_normal(bencher: &mut Bencher) {
    bencher.iter(|| to_normal(black_box(1.75)));
}
