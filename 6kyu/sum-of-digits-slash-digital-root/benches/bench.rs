#![feature(test)]

extern crate test;
use sum_of_digits_slash_digital_root::digital_root;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for n in [999_493_193, 992, 195] {
            black_box(digital_root(black_box(n)));
        }
    });
}
