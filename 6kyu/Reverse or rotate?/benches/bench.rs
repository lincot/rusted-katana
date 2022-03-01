#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box("1994033775182780067155464327690480265895");
    let c = black_box(5);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::revrot(s, c));
        }
    })
}
