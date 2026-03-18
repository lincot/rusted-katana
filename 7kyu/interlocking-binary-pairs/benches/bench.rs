#![feature(test)]

extern crate test;
use interlocking_binary_pairs::interlockable;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| interlockable(black_box(493_125_046_404), black_box(24584)));
}
