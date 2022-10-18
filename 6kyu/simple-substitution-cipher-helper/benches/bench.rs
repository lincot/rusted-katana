#![no_std]
#![feature(test)]

extern crate test;
use simple_substitution_cipher_helper::Cipher;
use test::{black_box, Bencher};

const MAP1: &str = "abcdefghijklmnopqrstuvwxyz";
const MAP2: &str = "etaoinshrdlucmfwypvbgkjqxz";

#[bench]
fn bench_new(bencher: &mut Bencher) {
    bencher.iter(|| Cipher::new(black_box(MAP1), black_box(MAP2)));
}

#[bench]
fn bench_encode(bencher: &mut Bencher) {
    bencher.iter(|| Cipher::new(MAP1, MAP2).decode(black_box("cnqjsdsfanappcxecxkchbapamofevesrecusvapgddwewhsnlpptvcqkxoluozgmmwlviwppbmbefpoyfobmhiivazdpfqdasal")));
}
