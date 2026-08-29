#![feature(test)]

extern crate test;
use death_by_coffee::coffee_limits;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(coffee_limits(black_box(1950), black_box(1), black_box(19)));
        black_box(coffee_limits(black_box(1965), black_box(12), black_box(11)));
    });
}
