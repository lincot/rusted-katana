#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const BEAST: &str = "медвед";
const DISH: &str = "мёд";

#[bench]
fn bench(bencher: &mut Bencher) {
    let beast = black_box(BEAST);
    let dish = black_box(DISH);

    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::feast(beast, dish));
        }
    })
}
