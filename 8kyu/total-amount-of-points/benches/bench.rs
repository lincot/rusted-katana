#![no_std]
#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use total_amount_of_points::points;

#[bench]
fn bench(bencher: &mut Bencher) {
    let games = black_box([
        "1:1".into(),
        "2:2".into(),
        "3:3".into(),
        "4:4".into(),
        "2:2".into(),
        "3:3".into(),
        "4:4".into(),
        "3:3".into(),
        "4:4".into(),
        "4:4".into(),
    ]);
    let games = black_box(&games);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(points(games));
        }
    });
}
