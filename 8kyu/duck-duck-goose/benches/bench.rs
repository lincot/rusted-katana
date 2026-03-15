#![feature(test)]

extern crate test;
use duck_duck_goose::{duck_duck_goose, Player};
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        duck_duck_goose(
            black_box(
                &["a", "b", "c", "d", "c", "e", "f", "g", "h", "z"].map(|name| Player { name }),
            ),
            black_box(30),
        )
    });
}
