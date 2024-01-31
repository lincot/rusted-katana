#![feature(test)]

extern crate test;
use do_i_get_a_bonus::bonus_time;
use test::{black_box, Bencher};

const SALARY: u64 = 10000;

#[bench]
fn bench_with_bonus(bencher: &mut Bencher) {
    bencher.iter(|| bonus_time(black_box(SALARY), black_box(true)));
}

#[bench]
fn bench_without_bonus(bencher: &mut Bencher) {
    bencher.iter(|| bonus_time(black_box(SALARY), black_box(false)));
}
