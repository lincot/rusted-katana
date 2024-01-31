#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use two_joggers::nbr_of_laps;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| nbr_of_laps(black_box(9496), black_box(4447)));
}
