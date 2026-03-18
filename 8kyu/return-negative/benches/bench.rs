#![feature(test)]

extern crate test;
use return_negative::make_negative;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(make_negative(black_box(-5)));
        black_box(make_negative(black_box(0)));
        black_box(make_negative(black_box(5)));
    });
}
