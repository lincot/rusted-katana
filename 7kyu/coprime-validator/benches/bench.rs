#![feature(test)]

extern crate test;
use coprime_validator::are_coprime;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(are_coprime(black_box(64), black_box(27)));
        black_box(are_coprime(black_box(35), black_box(10)));
    });
}
