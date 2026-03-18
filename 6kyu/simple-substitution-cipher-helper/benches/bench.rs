#![feature(test)]

extern crate test;
use simple_substitution_cipher_helper::Cipher;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    const MAP1: &str = "abcdefghijklmnopqrstuvwxyz";
    const MAP2: &str = "etaoinshrdlucmfwypvbgkjqxz";
    const STRING: &str = "cnqjsdsfanappcxecxkchbapamofevesrecusvapgddwewhsnlpptvcqkxoluozgmmwlviwppbmbefpoyfobmhiivazdpfqdasallasadqfpdzaviihmbofyopfebmbppwivlwmmgzouloxkqcvtpplnshwewddgpavsucersevefomapabhckxcexcppanafsdsjqnc";
    bencher.iter(|| {
        let cipher = Cipher::new(black_box(MAP1), black_box(MAP2));
        black_box(cipher.decode(black_box(STRING)));
    });
}
