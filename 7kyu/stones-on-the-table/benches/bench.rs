#![feature(test)]

extern crate test;
use stones_on_the_table::stones_to_remove;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..10 {
            black_box(stones_to_remove(black_box("RGBRGBRGGB")));
            black_box(stones_to_remove(black_box("RGGRGBBRGRR")));
            black_box(stones_to_remove(black_box("RRRRGGGGBBBB")));
        }
    });
}
