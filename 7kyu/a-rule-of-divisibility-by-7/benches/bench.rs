#![feature(test)]

extern crate test;
use a_rule_of_divisibility_by_7::seven;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| seven(black_box(477_557_102)));
}
