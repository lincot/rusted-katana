#![no_std]
#![feature(test)]

extern crate test;
use circle_cipher::{decode, encode};
use test::{black_box, Bencher};

#[bench]
fn bench_encode(bencher: &mut Bencher) {
    bencher.iter(|| encode(black_box("Вы решили перевести эту кату.".into())));
}

#[bench]
fn bench_decode(bencher: &mut Bencher) {
    bencher.iter(|| decode(black_box("В.ыу траекш иултиэ  пиетрсеев".into())));
}
