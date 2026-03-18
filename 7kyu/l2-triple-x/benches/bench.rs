#![feature(test)]

extern crate test;
use l2_triple_x::triple_x;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| triple_x(black_box("softXxxx kitty, warm kitty, xxxxx")));
}
