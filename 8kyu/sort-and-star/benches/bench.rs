#![feature(test)]

extern crate test;
use sort_and_star::two_sort;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        two_sort(black_box(&[
            "определённый",
            "интеграл",
            "представляет",
            "собой",
            "функцию",
            "ориентированного",
            "промежутка",
        ]))
    });
}
