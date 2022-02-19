#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const FROM: u16 = 1000;
const TO: u16 = 9000;

#[bench]
fn bench(bencher: &mut Bencher) {
    let from = black_box(FROM);
    let to = black_box(TO);

    bencher.iter(|| {
        for i in from..=to {
            black_box(solution::next_happy_year(black_box(i)));
        }
    })
}
