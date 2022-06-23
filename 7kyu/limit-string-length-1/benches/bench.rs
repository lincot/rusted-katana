#![feature(test)]

extern crate test;
use limit_string_length_1::solution;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let st = black_box("Тестовая строка");
    let limit = black_box(9);
    bencher.iter(|| solution(st, limit));
}
