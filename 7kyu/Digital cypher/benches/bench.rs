#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const MSG: &str = "In probability theory and statistics, variance is the expectation of the squared deviation of a random variable from its population mean or sample mean.";
const N: i32 = 54756;

#[bench]
fn bench(bencher: &mut Bencher) {
    let msg = black_box(MSG);
    let n = black_box(N);

    bencher.iter(|| solution::encode(msg.into(), n))
}
