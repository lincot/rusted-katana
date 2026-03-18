#![feature(test)]

extern crate test;
use is_there_an_odd_bit::any_odd;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| any_odd(black_box(2_863_311_530)));
}
