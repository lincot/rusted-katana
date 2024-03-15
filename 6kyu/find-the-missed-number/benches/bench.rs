#![feature(test)]

extern crate test;
use find_the_missed_number::find_number;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        find_number(
            black_box(1),
            black_box(21),
            black_box("1116122137143151617181920849510"),
        )
    });
}
