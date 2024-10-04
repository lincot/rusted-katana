#![feature(test)]

extern crate test;
use moves_in_squared_strings_iii::{diag_1_sym, rot_90_clock, selfie_and_diag1};
use test::{black_box, Bencher};

const S_ASCII: &str = "xhixpsfhrc\nznxkewfzaq\noegyxetoow\nfhprwpbeby\nsxocuozjsf\neurwonfoyr\nimxuyowpoq\nadvamwvqsg\nibsdohppat\nglcvwmeiqn";
const S_NONASCII: &str = "лвшпзчпцъл\nбнхчсгбтцч\nшгйрбшёвэх\nцбфбчажежл\nуаълхыйяэц\nэюдщыюгихн\nчнзпръцбьи\nфтгбфщегьа\nдймурикзяб\nшпхнгесиаг";

#[bench]
fn bench_diag_1_sym_ascii(bencher: &mut Bencher) {
    bencher.iter(|| diag_1_sym(black_box(S_ASCII)));
}

#[bench]
fn bench_rot_90_clock_ascii(bencher: &mut Bencher) {
    bencher.iter(|| rot_90_clock(black_box(S_ASCII)));
}

#[bench]
fn bench_selfie_and_diag1_ascii(bencher: &mut Bencher) {
    bencher.iter(|| selfie_and_diag1(black_box(S_ASCII)));
}

#[bench]
fn bench_diag_1_sym_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| diag_1_sym(black_box(S_NONASCII)));
}

#[bench]
fn bench_rot_90_clock_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| rot_90_clock(black_box(S_NONASCII)));
}

#[bench]
fn bench_selfie_and_diag1_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| selfie_and_diag1(black_box(S_NONASCII)));
}
