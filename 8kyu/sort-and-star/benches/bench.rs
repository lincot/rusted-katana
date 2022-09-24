#![no_std]
#![feature(test)]

extern crate test;
use sort_and_star::two_sort;
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
    bencher.iter(|| two_sort(arr));
}
