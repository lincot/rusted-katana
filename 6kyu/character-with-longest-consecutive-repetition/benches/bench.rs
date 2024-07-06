#![feature(test)]

extern crate test;
use character_with_longest_consecutive_repetition::longest_repetition;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| longest_repetition(black_box("неe, ну это жёёёёстко!!!")));
}
