#![feature(test)]

extern crate test;
use l1_set_alarm::set_alarm;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for employed in [false, true] {
            for vacation in [false, true] {
                set_alarm(black_box(employed), black_box(vacation));
            }
        }
    });
}
