#![feature(test)]

extern crate test;
use right_in_the_centre::is_in_middle;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(is_in_middle(black_box("abcabcabcabc")));
        black_box(is_in_middle(black_box("AAAabcBB")));
    });
}
