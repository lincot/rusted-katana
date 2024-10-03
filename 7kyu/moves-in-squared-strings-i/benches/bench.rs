#![feature(test)]

extern crate test;
use moves_in_squared_strings_i::{hor_mirror, vert_mirror};
use test::{black_box, Bencher};

const S_ASCII: &str = "lvypzchpcwl\nbnhcsgcch\nshgrbhvjeh\ncfwazhezhl\nulhyjjajec\njejhhjgihn\nchnzprcbyi\nftgbfshega\ndjmurkzjab\nshhngesiag";
const S_NONASCII: &str = "лвшпзчпцъл\nбнхчсгбтцч\nшгйрбшёвэх\nцбфбчажежл\nуаълхыйяэц\nэюдщыюгихн\nчнзпръцбьи\nфтгбфщегьа\nдймурикзяб\nшпхнгесиаг";

#[bench]
fn bench_hor_mirror_ascii(bencher: &mut Bencher) {
    bencher.iter(|| hor_mirror(black_box(S_ASCII.into())));
}

#[bench]
fn bench_hor_mirror_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| hor_mirror(black_box(S_NONASCII.into())));
}

#[bench]
fn bench_vert_mirror_ascii(bencher: &mut Bencher) {
    bencher.iter(|| vert_mirror(black_box(S_ASCII.into())));
}

#[bench]
fn bench_vert_mirror_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| vert_mirror(black_box(S_NONASCII.into())));
}
