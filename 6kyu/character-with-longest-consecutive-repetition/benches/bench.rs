#![feature(test)]

extern crate test;
use character_with_longest_consecutive_repetition::longest_repetition;
use test::{black_box, Bencher};

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| longest_repetition(black_box("nee, nu eto jeeeestko!!!")));
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| longest_repetition(black_box("неe, ну это жёёёёстко!!!")));
}
