#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use the_feast_of_many_beasts::feast;

#[bench]
fn bench(bencher: &mut Bencher) {
    let beast = black_box("медвед");
    let dish = black_box("мёд");
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(feast(beast, dish));
        }
    });
}
