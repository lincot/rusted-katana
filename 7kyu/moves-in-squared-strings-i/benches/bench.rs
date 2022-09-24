#![no_std]
#![feature(test)]

extern crate alloc;
extern crate test;
use alloc::string::ToString;
use moves_in_squared_strings_i::{hor_mirror, vert_mirror};
use test::{black_box, Bencher};

const S: &str = "лвшпзчпцъл\nбнхчсгбтцч\nшгйрбшёвэх\nцбфбчажежл\nуаълхыйяэц\nэюдщыюгихн\nчнзпръцбьи\nфтгбфщегьа\nдймурикзяб\nшпхнгесиаг";

#[bench]
fn bench_hor_mirror(bencher: &mut Bencher) {
    let s = black_box(S.to_string());
    bencher.iter(|| hor_mirror(s.clone()));
}

#[bench]
fn bench_vert_mirror(bencher: &mut Bencher) {
    let s = black_box(S.to_string());
    bencher.iter(|| vert_mirror(s.clone()));
}
