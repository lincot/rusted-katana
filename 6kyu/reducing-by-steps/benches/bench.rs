#![feature(test)]

extern crate test;
use core::array;
use reducing_by_steps::{gcdi, lcmu, oper_array, som};
use test::{black_box, Bencher};

#[bench]
fn bench_gcdi(bencher: &mut Bencher) {
    bencher.iter(|| gcdi(black_box(-1_000_000), black_box(-(1 << 20))));
}

#[bench]
fn bench_lcmu(bencher: &mut Bencher) {
    bencher.iter(|| lcmu(black_box(-1_000_000), black_box(-(1 << 20))));
}

#[bench]
fn bench_oper_array(bencher: &mut Bencher) {
    let a: [_; 1000] = array::from_fn(|i| i as i64 * 5);
    bencher.iter(|| oper_array(som, black_box(&a), black_box(5)));
}
