#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const ST: &str = "Тестовая строка";
const LIMIT: usize = 9;

#[bench]
fn bench(bencher: &mut Bencher) {
    let st = black_box(ST);
    let limit = black_box(LIMIT);

    bencher.iter(|| solution::solution(st, limit))
}
