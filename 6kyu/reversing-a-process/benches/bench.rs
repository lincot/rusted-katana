#![feature(test)]

extern crate test;
use reversing_a_process::decode;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| decode(black_box("10559625hbkeohysnztuuqdznnkkcgjndbujej")));
}
