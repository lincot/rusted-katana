#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const S: &str = "01000000X000X011X0X";

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box(S);

    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::infected(s));
        }
    })
}
