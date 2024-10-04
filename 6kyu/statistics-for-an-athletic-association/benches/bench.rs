#![feature(test)]

extern crate test;
use statistics_for_an_athletic_association::stati;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        stati(black_box(
            "02|15|59, 2|47|16, 02|17|20, 2|32|34, 2|17|17, 2|22|00, 2|31|41",
        ))
    });
}
