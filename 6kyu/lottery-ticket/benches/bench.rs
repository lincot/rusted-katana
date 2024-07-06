#![feature(test)]

extern crate test;
use lottery_ticket::bingo;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        bingo(
            black_box(&[
                ("ABC", 65),
                ("HGR", 74),
                ("BYHT", 74),
                ("HGTYRE", 74),
                ("BE", 66),
                ("JKTY", 74),
            ]),
            black_box(5),
        )
    });
}
