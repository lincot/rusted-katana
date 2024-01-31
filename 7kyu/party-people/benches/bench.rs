#![feature(test)]

extern crate test;
use party_people::party_people;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        party_people(black_box(&[
            0, 2, 3, 5, 6, 6, 6, 7, 11, 12, 13, 14, 16, 19, 20,
        ]))
    });
}
