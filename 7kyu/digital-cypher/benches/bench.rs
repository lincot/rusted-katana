#![feature(test)]

extern crate test;
use digital_cypher::encode;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let msg = black_box("In probability theory and statistics, variance is the expectation of the squared deviation of a random variable from its population mean or sample mean.");
    let n = black_box(543_756);
    bencher.iter(|| encode(msg.into(), n));
}
