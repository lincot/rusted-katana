#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let arr = black_box(&[
        "определённый",
        "интеграл",
        "представляет",
        "собой",
        "функцию",
        "ориентированного",
        "промежутка",
    ]);
    bencher.iter(|| solution::two_sort(arr));
}
