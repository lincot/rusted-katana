#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use uncollapse_digits::uncollapse;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| uncollapse(black_box("zeroonetwothreefourfivesixseveneightninezero")));
}
