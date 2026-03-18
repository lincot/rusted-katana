#![feature(test)]

extern crate test;
use round_up_to_the_next_multiple_of_5::round_to_next_5;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| round_to_next_5(black_box(121)));
}
