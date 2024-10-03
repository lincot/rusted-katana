#![feature(test)]

extern crate test;
use digital_cypher::encode;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    const MSG: &str = "In probability theory and statistics, variance is the expectation of the squared deviation of a random variable from its population mean or sample mean.";

    bencher.iter(|| encode(black_box(MSG.into()), black_box(543_756)));
}
