#![feature(test)]

extern crate test;
use shortest_steps_to_a_number::shortest_steps_to_num;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| shortest_steps_to_num(black_box(8314)));
}
