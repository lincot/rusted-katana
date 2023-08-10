#![no_std]
#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use the_fusc_function_part_1::fusc;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(fusc(black_box(2_376_499_510)));
        }
    });
}
