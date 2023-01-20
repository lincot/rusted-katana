#![no_std]
#![feature(test)]

extern crate test;
use count_the_monkeys::monkey_count;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for n in 0..1000 {
            black_box(monkey_count(black_box(n)));
        }
    });
}
