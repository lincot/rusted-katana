#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let p0 = black_box(1_500_000);
    let percent = black_box(2.5);
    let aug = black_box(10_000);
    let p = black_box(2_000_000);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::nb_year(p0, percent, aug, p));
        }
    });
}
