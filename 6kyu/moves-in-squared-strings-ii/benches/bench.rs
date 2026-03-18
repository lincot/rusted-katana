#![feature(test)]

extern crate test;
use moves_in_squared_strings_ii::{rot, selfie_and_rot};
use test::{Bencher, black_box};

const S_ASCII: &str = "xhixpsfhrc\nznxkewfzaq\noegyxetoow\nfhprwpbeby\nsxocuozjsf\neurwonfoyr\nimxuyowpoq\nadvamwvqsg\nibsdohppat\nglcvwmeiqn";
const S_NONASCII: &str = "лвшпзчпцъл\nбнхчсгбтцч\nшгйрбшёвэх\nцбфбчажежл\nуаълхыйяэц\nэюдщыюгихн\nчнзпръцбьи\nфтгбфщегьа\nдймурикзяб\nшпхнгесиаг";

#[bench]
fn bench_rot_ascii(bencher: &mut Bencher) {
    bencher.iter(|| rot(black_box(S_ASCII)));
}

#[bench]
fn bench_selfie_and_rot_ascii(bencher: &mut Bencher) {
    bencher.iter(|| selfie_and_rot(black_box(S_ASCII)));
}

#[bench]
fn bench_rot_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| rot(black_box(S_NONASCII)));
}

#[bench]
fn bench_selfie_and_rot_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| selfie_and_rot(black_box(S_NONASCII)));
}
