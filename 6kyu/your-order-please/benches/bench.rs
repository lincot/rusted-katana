#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use your_order_please::order;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| order(black_box("d4o dru7nken sh2all w5ith s8ailor wha1t 3we a6")));
}
