#![feature(test)]

extern crate test;
use esolang_minibitmove::interpreter;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    const TAPE: &str = "01001101010101110100010101011100101011001101000111100101011111100001111110100101011101000100001001000101101011001111001011001000110010000100001111010001101011110100100001010010011111110010101000011110110010010001101000010100101011100001111000010101011000110010000100110001001111101111100101001100001101001010001100111010011111111111001100000110001010110100010010111110000111011111100001001100110001100001101111111100001011110001010100010011010101011110011101010110101100010011000011101001010011001";
    const DATA: &str = "0110010101100011111111010101010001101001000101110000110110010000011111011110011001110011100011010000001001010101001011010101000111110001001001100010110110101101010001001001011010001110010011100101011011001110101001110111001001110101011100010001100011010101111110111100111010011101001111101101001010010111011011100100001110100000100001000000000010000111011001011";

    bencher.iter(|| interpreter(black_box(TAPE), black_box(DATA)));
}
