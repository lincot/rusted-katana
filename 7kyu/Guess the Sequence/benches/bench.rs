#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const FROM: u8 = 1;
const TO: u8 = 100;

#[bench]
fn bench(bencher: &mut Bencher) {
    let from = black_box(FROM);
    let to = black_box(TO);

    bencher.iter(|| {
        for x in from..=to {
            black_box(solution::sequence(black_box(x)));
        }
    })
}
