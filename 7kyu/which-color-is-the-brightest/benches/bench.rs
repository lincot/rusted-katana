#![no_std]
#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use which_color_is_the_brightest::brightest;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        brightest(black_box(&[
            "#408E52".into(),
            "#16BE69".into(),
            "#E720DB".into(),
            "#663280".into(),
            "#BF6255".into(),
            "#582F29".into(),
            "#95BB13".into(),
            "#5F816E".into(),
            "#4A0BEE".into(),
            "#97E20F".into(),
            "#B70E2F".into(),
            "#C68B6A".into(),
            "#FFE716".into(),
            "#43DD35".into(),
        ]))
    });
}
