#![no_std]
#![feature(test)]

extern crate test;
use solution_16_plus_18_equals_214::add;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| add(black_box(645_612_459), black_box(1_837_889_123)));
}
