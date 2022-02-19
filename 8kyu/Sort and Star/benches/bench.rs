#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const ARR: [&str; 7] = [
    "определённый",
    "интеграл",
    "представляет",
    "собой",
    "функцию",
    "ориентированного",
    "промежутка",
];

#[bench]
fn bench(bencher: &mut Bencher) {
    let arr = black_box(&ARR);

    bencher.iter(|| solution::two_sort(arr))
}
