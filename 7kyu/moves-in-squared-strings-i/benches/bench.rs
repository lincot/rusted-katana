#![feature(test)]

extern crate test;
use moves_in_squared_strings_i::{hor_mirror, vert_mirror};
use test::{black_box, Bencher};

const S: &str = "лвшпзчпцъл\nбнхчсгбтцч\nшгйрбшёвэх\nцбфбчажежл\nуаълхыйяэц\nэюдщыюгихн\nчнзпръцбьи\nфтгбфщегьа\nдймурикзяб\nшпхнгесиаг";

#[bench]
fn bench_hor_mirror(bencher: &mut Bencher) {
    let s = black_box(S);
    bencher.iter(|| hor_mirror(s.into()));
}

#[bench]
fn bench_vert_mirror(bencher: &mut Bencher) {
    let s = black_box(S);
    bencher.iter(|| vert_mirror(s.into()));
}
