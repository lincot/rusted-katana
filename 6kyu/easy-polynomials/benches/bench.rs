#![feature(test)]

extern crate test;
use easy_polynomials::easy_polynomials;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(easy_polynomials(black_box(10)));
        black_box(easy_polynomials(black_box(11)));
    });
}
