#![feature(test)]

extern crate test;
use find_the_position::position;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for c in 'a'..='z' {
            black_box(position(black_box(c)));
        }
    });
}
