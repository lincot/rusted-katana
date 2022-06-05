#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let language = black_box("swedish");
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::greet(language));
        }
    });
}
