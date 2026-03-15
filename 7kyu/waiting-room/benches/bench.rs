#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use waiting_room::last_chair;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| last_chair(black_box(10)));
}
