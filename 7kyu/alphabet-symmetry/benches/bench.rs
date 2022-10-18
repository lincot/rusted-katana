#![no_std]
#![feature(test)]

extern crate test;
use alphabet_symmetry::solve;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solve(black_box(&[
                "IAMDEFANDJKL".into(),
                "thedefgh".into(),
                "xyzDEFghijabc".into(),
                "encode".into(),
                "abc".into(),
                "abcdefghsomething".into(),
                "somethingsomethingsomethingsomethingsomethingsomething".into(),
            ])));
        }
    });
}
