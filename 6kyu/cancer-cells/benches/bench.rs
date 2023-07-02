#![no_std]
#![feature(test)]

extern crate test;
use cancer_cells::cut_cancer_cells;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        cut_cancer_cells(black_box(
            "acCcbacCcbacCcbacCcbacCcbacCcbacCcbBCEBCEBCENcENcECAACCAACCAACCaaCCaaCCCCCCacbacbacb",
        ));
    });
}
