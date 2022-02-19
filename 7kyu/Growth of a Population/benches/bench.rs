#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const P0: i32 = 1500000;
const PERCENT: f64 = 2.5;
const AUG: i32 = 10000;
const P: i32 = 2000000;

#[bench]
fn bench(bencher: &mut Bencher) {
    let p0 = black_box(P0);
    let percent = black_box(PERCENT);
    let aug = black_box(AUG);
    let p = black_box(P);

    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::nb_year(p0, percent, aug, p));
        }
    })
}
