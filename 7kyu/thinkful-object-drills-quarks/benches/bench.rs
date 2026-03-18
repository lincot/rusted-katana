#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use thinkful_object_drills_quarks::Quark;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut quark1 = Quark::new("red", "up");
        let mut quark2 = Quark::new("blue", "strange");
        black_box(&mut quark1).interact(black_box(&mut quark2));
        (quark1, quark2)
    });
}
