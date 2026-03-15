#![feature(test)]

extern crate test;
use get_nth_even_number::nth_even;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| nth_even(black_box(20)));
}
