#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const FROM: u32 = 1;
const TO: u32 = 1_000_000;

#[bench]
fn bench(bencher: &mut Bencher) {
    let from = black_box(FROM);
    let to = black_box(TO);

    bencher.iter(|| {
        for i in from..=to {
            black_box(solution::doubleton(black_box(i)));
        }
    })
}
