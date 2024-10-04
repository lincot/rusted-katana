#![feature(test)]

extern crate test;
use if_you_cant_sleep_just_count_sheep::count_sheep;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| count_sheep(black_box(if cfg!(miri) { 10 } else { 1000 })));
}
