#![feature(test)]

extern crate test;
use corner_circle::corner_circle;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    // somehow a pretty much identical most upvoted solution is faster
    // with a single call
    bencher.iter(|| {
        black_box(corner_circle(black_box(17.)));
        black_box(corner_circle(black_box(18.)));
        black_box(corner_circle(black_box(19.)));
    });
}
