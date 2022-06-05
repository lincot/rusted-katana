#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const MAP1: &str = "abcdefghijklmnopqrstuvwxyz";
const MAP2: &str = "etaoinshrdlucmfwypvbgkjqxz";

#[bench]
fn bench_new(bencher: &mut Bencher) {
    let map1 = black_box(MAP1);
    let map2 = black_box(MAP2);
    bencher.iter(|| solution::Cipher::new(map1, map2));
}

#[bench]
fn bench_encode(bencher: &mut Bencher) {
    let cipher = solution::Cipher::new(MAP1, MAP2);
    let string = black_box("cnqjsdsfanappcxecxkchbapamofevesrecusvapgddwewhsnlpptvcqkxoluozgmmwlviwppbmbefpoyfobmhiivazdpfqdasal");
    bencher.iter(|| cipher.decode(string));
}
