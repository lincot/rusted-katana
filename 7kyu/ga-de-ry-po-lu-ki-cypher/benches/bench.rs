#![feature(test)]

extern crate test;
use ga_de_ry_po_lu_ki_cypher::encode;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        encode(black_box(
            "Ala has a cat ABCD gaderypoluki Gug hgs g cgt GBCE agedyropulik",
        ))
    });
}
