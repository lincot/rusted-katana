#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use what_is_between::between;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| between(black_box(-3000), black_box(3000)));
}
