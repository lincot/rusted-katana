#![feature(test)]

extern crate test;
use holiday_vi_shark_pontoon::shark;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        shark(
            black_box(5.),
            black_box(6.),
            black_box(2.),
            black_box(2.5),
            black_box(true),
        )
    });
}
