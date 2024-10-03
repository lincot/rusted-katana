#![feature(test)]

extern crate test;
use circle_cipher::{decode, encode};
use test::{black_box, Bencher};

#[bench]
fn bench_encode_ascii(bencher: &mut Bencher) {
    bencher.iter(|| encode(black_box("You have chosen to translate this kata.".into())));
}

#[bench]
fn bench_encode_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| encode(black_box("Вы решили перевести эту кату.".into())));
}

#[bench]
fn bench_decode_ascii(bencher: &mut Bencher) {
    bencher.iter(|| decode(black_box("Y.oaut ahka vsei hcth oesteanl stnoa rt".into())));
}

#[bench]
fn bench_decode_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| decode(black_box("В.ыу траекш иултиэ  пиетрсеев".into())));
}
