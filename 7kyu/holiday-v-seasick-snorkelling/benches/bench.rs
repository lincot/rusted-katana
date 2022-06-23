#![feature(test)]

extern crate test;
use holiday_v_seasick_snorkelling::sea_sick;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let sea = black_box("_~~~~~~~_~__~______~~__~~_~~");
    bencher.iter(|| sea_sick(sea));
}
