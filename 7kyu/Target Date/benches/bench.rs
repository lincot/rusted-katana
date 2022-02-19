#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const A0: i32 = 4281;
const A: i32 = 5087;
const P: i32 = 2;

#[bench]
fn bench(bencher: &mut Bencher) {
    let a0 = black_box(A0);
    let a = black_box(A);
    let p = black_box(P);

    bencher.iter(|| solution::date_nb_days(a0, a, p))
}
