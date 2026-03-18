#![feature(test)]

extern crate test;
use lojban_numbers::convert_lojban;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| convert_lojban(black_box("renonorecivozenovocirepa")));
}
