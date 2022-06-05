#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const S: &str = "лвшпзчпцъл\nбнхчсгбтцч\nшгйрбшёвэх\nцбфбчажежл\nуаълхыйяэц\nэюдщыюгихн\nчнзпръцбьи\nфтгбфщегьа\nдймурикзяб\nшпхнгесиаг";

#[bench]
fn bench_hor_mirror(bencher: &mut Bencher) {
    let s = black_box(S);
    bencher.iter(|| solution::hor_mirror(s.into()));
}

#[bench]
fn bench_vert_mirror(bencher: &mut Bencher) {
    let s = black_box(S);
    bencher.iter(|| solution::vert_mirror(s.into()));
}
