#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use welcome::greet;

#[bench]
fn bench(bencher: &mut Bencher) {
    let language = black_box("swedish");
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(greet(language));
        }
    });
}
