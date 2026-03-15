#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use the_pony_express::riders;

#[bench]
fn bench(bencher: &mut Bencher) {
    // most upvoted solutions needs Vec
    let stations = vec![6, 24, 6, 8, 28, 8, 23, 47, 17, 29, 37, 18, 40, 49];
    bencher.iter(|| riders(black_box(&stations)));
}
