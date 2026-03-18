#![feature(test)]

extern crate test;
use pack_some_chocolates::make_chocolates;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for (small, big, goal) in [
            (4, 1, 13),
            (4, 1, 14),
            (2, 1, 7),
            (3, 1, 6),
            (2, 0, 6),
            (8, 0, 7),
            (8, 15, 3),
        ] {
            black_box(make_chocolates(
                black_box(small),
                black_box(big),
                black_box(goal),
            ));
        }
    });
}
