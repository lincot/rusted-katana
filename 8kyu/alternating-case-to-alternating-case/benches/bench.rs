#![feature(test)]

extern crate test;
use alternating_case_to_alternating_case::to_alternating_case;
use test::{black_box, Bencher};

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| to_alternating_case(black_box("hElLo WoRlD 2020")));
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| to_alternating_case(black_box("пРиВеТ МииР 2020")));
}
