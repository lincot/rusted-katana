#![feature(test)]

extern crate test;
use pillars::pillars;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    // somehow if we only leave one call, speedup over the identical most
    // upvoted solution (located in a separate crate) drops to 0.8
    bencher.iter(|| {
        black_box(pillars(black_box(1), black_box(10), black_box(10)));
        black_box(pillars(black_box(2), black_box(20), black_box(25)));
        black_box(pillars(black_box(11), black_box(15), black_box(30)));
    });
}
