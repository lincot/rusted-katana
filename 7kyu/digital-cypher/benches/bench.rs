#![no_std]
#![feature(test)]

extern crate alloc;
extern crate test;
use alloc::string::ToString;
use digital_cypher::encode;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let msg = black_box("In probability theory and statistics, variance is the expectation of the squared deviation of a random variable from its population mean or sample mean.".to_string());
    let n = black_box(543_756);
    bencher.iter(|| encode(msg.clone(), n));
}
