#![feature(test)]

extern crate test;
use take_the_derivative::derive;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| derive(black_box(20), black_box(30)));
}
