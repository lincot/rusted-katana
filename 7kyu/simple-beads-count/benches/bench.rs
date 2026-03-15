#![feature(test)]

extern crate test;
use simple_beads_count::count_red_beads;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| count_red_beads(black_box(5)));
}
