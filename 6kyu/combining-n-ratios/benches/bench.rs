#![feature(test)]

extern crate test;
use combining_n_ratios::ratio_combinator;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        ratio_combinator(black_box(&[
            "240:170", "17:38", "304:216", "216:312", "390:40", "36:432", "48:47", "470:180",
            "144:96", "96:200", "225:162", "144:368", "184:172", "430:350", "210:114", "133:266",
            "190:85", "170:420", "210:105", "147:252",
        ]))
    });
}
