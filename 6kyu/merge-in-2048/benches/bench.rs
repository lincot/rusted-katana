#![feature(test)]

extern crate test;
use merge_in_2048::merge;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        merge(black_box(&[
            8, 4, 4, 8, 0, 2, 32, 8, 2, 4, 4, 16, 32, 16, 16, 0,
        ]))
    });
}
