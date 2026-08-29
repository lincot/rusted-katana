#![feature(test)]

extern crate test;
use perimeter_sequence::perimeter_sequence;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| perimeter_sequence(black_box(4), black_box(4)));
}
