#![feature(test)]

extern crate test;
use grasshopper_messi_goals::total_goals;
use test::Bencher;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| total_goals);
}
