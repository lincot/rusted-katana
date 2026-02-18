#![feature(test)]

extern crate test;
use can_santa_save_christmas::determine_time;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| determine_time(black_box(&["04:30:00", "02:00:00", "01:30:00", "16:00:00"])));
}
