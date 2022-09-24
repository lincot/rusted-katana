#![no_std]
#![feature(test)]

extern crate test;
use leaderboard_climbers::leaderboard_sort;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let leaderbord = [
        "John".into(),
        "Brian".into(),
        "Jim".into(),
        "Dave".into(),
        "Fred".into(),
        "Kevin".into(),
        "Larry".into(),
    ];
    let leaderbord = black_box(&leaderbord);
    let changes = [
        "Dave +1".into(),
        "Fred +4".into(),
        "Brian -1".into(),
        "Kevin -1".into(),
    ];
    let changes = black_box(&changes);
    bencher.iter(|| leaderboard_sort(leaderbord, changes));
}
