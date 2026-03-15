#![feature(test)]

extern crate test;
use color_choice::check_choose;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for (m, n) in [
            (6, 4),
            (4, 4),
            (4, 2),
            (35, 7),
            (36, 7),
            (184_756, 20),
            (184_756, 10),
            (3_268_760, 25),
            (155_117_520, 30),
            (155_117_530, 30),
            (6_540_715_896, 48),
            (6_540_715_897, 48),
            (344_867_425_584, 45),
        ] {
            black_box(check_choose(black_box(m), black_box(n)));
        }
    });
}
