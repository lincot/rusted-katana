#![feature(test)]

extern crate test;
use counting_duplicates::count_duplicates;
use test::{Bencher, black_box};

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| {
        count_duplicates(black_box(
            "F4tZJDa27ZSYrXufAmp3dbmIODR0WMYjXwasvt0JMAlXE91MKKMCTngn2RdpUyw4SK",
        ))
    });
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| {
        count_duplicates(black_box(
            "Ф4тЗЙДа27ЗСИрХуфАмп3дбмІОДР0ВМИйХвасвт0ЙМАлХЕ91МККМЦТнґн2РдпУив4СК",
        ))
    });
}
