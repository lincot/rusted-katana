#![no_std]
#![feature(test)]

extern crate test;
use do_i_get_a_bonus::bonus_time;
use test::{black_box, Bencher};

const SALARY: u64 = 10000;

#[bench]
fn bench_with_bonus(bencher: &mut Bencher) {
    let salary = black_box(SALARY);
    let bonus = black_box(true);
    bencher.iter(|| bonus_time(salary, bonus));
}

#[bench]
fn bench_without_bonus(bencher: &mut Bencher) {
    let salary = black_box(SALARY);
    let bonus = black_box(false);
    bencher.iter(|| bonus_time(salary, bonus));
}
