#![no_std]
#![feature(test)]

extern crate test;
use leaderboard_climbers::leaderboard_sort;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        leaderboard_sort(
            black_box(&[
                "John".into(),
                "Brian".into(),
                "Jim".into(),
                "Dave".into(),
                "Fred".into(),
                "Kevin".into(),
                "Larry".into(),
            ]),
            black_box(&[
                "Dave +1".into(),
                "Fred +4".into(),
                "Brian -1".into(),
                "Kevin -1".into(),
            ]),
        )
    });
}
