#![no_std]
#![feature(test)]

extern crate test;
use count_the_monkeys::monkey_count;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| monkey_count(black_box(30000)));
}
