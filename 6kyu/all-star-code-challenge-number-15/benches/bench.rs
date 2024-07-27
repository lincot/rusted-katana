#![feature(test)]

extern crate test;
use all_star_code_challenge_number_15::rotate;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| rotate(black_box("Привет всем!")));
}
