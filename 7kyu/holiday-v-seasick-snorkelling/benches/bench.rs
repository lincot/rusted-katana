#![no_std]
#![feature(test)]

extern crate test;
use holiday_v_seasick_snorkelling::sea_sick;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| sea_sick(black_box("_~~~~~~~_~__~______~~__~~_~~")));
}
