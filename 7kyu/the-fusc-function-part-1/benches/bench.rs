#![no_std]
#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use the_fusc_function_part_1::fusc;

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(2_376_499_510);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(fusc(n));
        }
    });
}
