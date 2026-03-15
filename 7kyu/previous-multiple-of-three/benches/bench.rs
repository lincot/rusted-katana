#![feature(test)]

extern crate test;
use previous_multiple_of_three::prev_mult_of_three;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| prev_mult_of_three(black_box(952_406)));
}
