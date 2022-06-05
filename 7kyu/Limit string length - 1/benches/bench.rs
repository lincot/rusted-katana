#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let st = black_box("Тестовая строка");
    let limit = black_box(9);
    bencher.iter(|| solution::solution(st, limit));
}
