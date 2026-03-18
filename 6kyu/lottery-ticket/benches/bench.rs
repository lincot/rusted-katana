#![feature(test)]

extern crate test;
use lottery_ticket::bingo;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        bingo(
            black_box(&[("HGTYRE", 74), ("BE", 66), ("JKTY", 74)]),
            black_box(3),
        )
    });
}
