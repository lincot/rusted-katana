#![no_std]
#![feature(test)]

extern crate test;
use alternating_case_to_alternating_case::to_alternating_case;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box("пРиВеТ МииР 2020");
    bencher.iter(|| to_alternating_case(s));
}
