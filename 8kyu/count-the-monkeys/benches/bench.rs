#![feature(test)]

extern crate test;
use count_the_monkeys::monkey_count;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| monkey_count(black_box(500)));
}
