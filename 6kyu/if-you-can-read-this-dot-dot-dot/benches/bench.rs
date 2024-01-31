#![feature(test)]

extern crate test;
use if_you_can_read_this_dot_dot_dot::to_nato;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| to_nato(black_box("If, you can read? go for it!")));
}
