#![no_std]
#![feature(test)]

extern crate test;
use return_the_first_m_multiples_of_n::multiples;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| multiples(black_box(100), black_box(3.15)));
}
