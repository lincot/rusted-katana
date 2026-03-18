#![feature(test)]

extern crate test;
use string_doubles::doubles;
use test::{Bencher, black_box};

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| {
        doubles(black_box(if cfg!(miri) {
            "zzzzykkkd"
        } else {
            "xxbnnnnnyaaaaamrrrmooomqqqqjabbbzz"
        }))
    });
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| {
        doubles(black_box(if cfg!(miri) {
            "ззззыкккд"
        } else {
            "ххбннннныааааамвидхвыдуслььььиддхпй"
        }))
    });
}
