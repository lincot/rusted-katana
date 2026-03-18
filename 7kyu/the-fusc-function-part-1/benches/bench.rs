#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use the_fusc_function_part_1::fusc;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| fusc(black_box(2_376_499_510)));
}
