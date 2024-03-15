#![feature(test)]

extern crate test;
use goldbachs_conjecture::check_goldbach;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(check_goldbach(black_box(111_788)));
        black_box(check_goldbach(black_box(12_345_678)));
        black_box(check_goldbach(black_box(12_345_680)));
        black_box(check_goldbach(black_box(12_345_682)));
    });
}
