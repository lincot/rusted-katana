#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use unique_strings::uniq_count;

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| {
        uniq_count(black_box(
            "Nefertitiandherhusbandwereknownfortheirradicaloverhaulofstatereligiouspolicy",
        ))
    });
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| {
        uniq_count(black_box(
            "ВремяправленияЭхнатонаиНефертитиизвестноеознаменовалосьрелигиознойреформой",
        ))
    });
}
