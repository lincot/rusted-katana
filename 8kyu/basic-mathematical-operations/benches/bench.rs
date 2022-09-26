#![no_std]
#![feature(test)]

extern crate test;
use basic_mathematical_operations::basic_op;
use test::{black_box, Bencher};

#[bench]
fn bench_estonian(bencher: &mut Bencher) {
    let operator = black_box('-');
    let value1 = black_box(256);
    let value2 = black_box(32);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(basic_op(operator, value1, value2));
        }
    });
}
