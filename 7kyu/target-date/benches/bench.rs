#![no_std]
#![feature(test)]

extern crate test;
use target_date::date_nb_days;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let a0 = black_box(4281);
    let a = black_box(5087);
    let p = black_box(2);
    bencher.iter(|| date_nb_days(a0, a, p));
}
