#![feature(test)]

extern crate test;
use mexican_wave::wave;
use test::{Bencher, black_box};

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    const S: &str = "Many recent philosophers have also diverged";

    bencher.iter(|| wave(black_box(S)));
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    const S: &str = "Дионисий Младший снова призывает Платона";

    bencher.iter(|| wave(black_box(S)));
}
