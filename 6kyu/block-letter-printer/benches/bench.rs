#![feature(test)]

extern crate test;
use block_letter_printer::block_print;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| block_print(black_box("Many recent philosophers have also diverged from what some would describe as ideals characteristic of traditional Platonism")));
}
