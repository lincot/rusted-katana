#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use will_you_make_it::zero_fuel;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| zero_fuel(black_box(200), black_box(5), black_box(100)));
}
