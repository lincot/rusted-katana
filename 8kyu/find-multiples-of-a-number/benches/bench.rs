#![no_std]
#![feature(test)]

extern crate test;
use find_multiples_of_a_number::find_multiples;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| find_multiples(black_box(11), black_box(1000)));
}
