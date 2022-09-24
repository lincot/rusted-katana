#![no_std]
#![feature(test)]

extern crate test;
use if_you_cant_sleep_just_count_sheep::count_sheep;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(1000);
    bencher.iter(|| count_sheep(n));
}
