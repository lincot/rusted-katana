#![feature(test)]

extern crate test;
use handshake_problem::get_participants;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| get_participants(black_box(500)));
}
