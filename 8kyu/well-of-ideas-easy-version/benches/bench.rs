#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use well_of_ideas_easy_version::well;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(well(black_box(&["bad", "bad", "bad"])));
        black_box(well(black_box(&["good", "bad", "bad", "bad"])));
        black_box(well(black_box(&[
            "good", "bad", "bad", "bad", "bad", "good", "bad", "bad", "good",
        ])));
    });
}
