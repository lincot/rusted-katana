#![feature(test)]

extern crate test;
use target_date::date_nb_days;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| date_nb_days(black_box(4281), black_box(5087), black_box(2)));
}
