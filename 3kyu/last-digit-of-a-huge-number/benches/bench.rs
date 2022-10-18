#![no_std]
#![feature(test)]

extern crate test;
use last_digit_of_a_huge_number::last_digit;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(last_digit(black_box(&[0, 1, 1, 1, 0, 0, 1, 1, 1, 1])));
            black_box(last_digit(black_box(&[2, 1, 1, 1, 0, 2, 0, 2, 2, 1])));
            black_box(last_digit(black_box(&[
                981_329, 105_498, 394_157, 778_765, 882_090, 703_537, 522_924, 19_299, 760_341,
                639_691,
            ])));
            black_box(last_digit(black_box(&[
                69_934, 615_179, 275_246, 438_762, 594_778, 118_585, 151_887, 518_956, 710_732,
                909_914,
            ])));
            black_box(last_digit(black_box(&[
                852_184, 58_078, 391_926, 885_300, 575_762, 674_521, 535_118, 608_476, 412_105,
                148_570,
            ])));
            black_box(last_digit(black_box(&[
                251_600, 870_294, 656_893, 528_745, 934_659, 730_929, 613_313, 312_500, 743_854,
                586_209,
            ])));
        }
    });
}
