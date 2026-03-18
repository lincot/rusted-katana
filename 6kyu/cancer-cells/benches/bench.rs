#![feature(test)]

extern crate test;
use cancer_cells::cut_cancer_cells;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        cut_cancer_cells(black_box(
            "acCcbacCcbacCcbacCcbacCcbacCcbacCcbBCEBCEBCENcENcECAACCAACCAACCaaCCaaCCCCCCacbacbacb",
        ))
    });
}
