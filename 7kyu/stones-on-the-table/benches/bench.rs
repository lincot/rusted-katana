#![feature(test)]

extern crate test;
use stones_on_the_table::stones_to_remove;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| stones_to_remove(black_box("RGBRGBRGGB")));
}
