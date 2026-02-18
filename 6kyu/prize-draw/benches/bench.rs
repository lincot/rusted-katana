#![feature(test)]

extern crate test;
use prize_draw::rank;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        rank(
            black_box("Addison,Jayden,Sofia,Michael,Andrew,Lily,Benjamin"),
            black_box(vec![4, 2, 1, 4, 3, 1, 2]),
            black_box(4),
        )
    });
}
