#![feature(test)]

extern crate test;
use count_the_monkeys::monkey_count;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(30000);
    bencher.iter(|| monkey_count(n));
}
