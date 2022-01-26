#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const SALARY: u64 = 10000;

#[bench]
fn bench_with_bonus(b: &mut Bencher) {
    let salary = black_box(SALARY);
    let bonus = black_box(true);

    b.iter(|| solution::bonus_time(salary, bonus))
}

#[bench]
fn bench_without_bonus(b: &mut Bencher) {
    let salary = black_box(SALARY);
    let bonus = black_box(false);

    b.iter(|| solution::bonus_time(salary, bonus))
}
