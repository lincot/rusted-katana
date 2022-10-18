#![no_std]
#![feature(test)]

extern crate test;
use moves_in_squared_strings_i::{hor_mirror, vert_mirror};
use test::{black_box, Bencher};

const S: &str = "лвшпзчпцъл\nбнхчсгбтцч\nшгйрбшёвэх\nцбфбчажежл\nуаълхыйяэц\nэюдщыюгихн\nчнзпръцбьи\nфтгбфщегьа\nдймурикзяб\nшпхнгесиаг";

#[bench]
fn bench_hor_mirror(bencher: &mut Bencher) {
    bencher.iter(|| hor_mirror(black_box(S.into())));
}

#[bench]
fn bench_vert_mirror(bencher: &mut Bencher) {
    bencher.iter(|| vert_mirror(black_box(S.into())));
}
