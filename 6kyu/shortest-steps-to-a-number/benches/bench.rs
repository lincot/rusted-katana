#![no_std]
#![feature(test)]

extern crate test;
use shortest_steps_to_a_number::shortest_steps_to_num;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(shortest_steps_to_num(black_box(8314)));
        }
    });
}
