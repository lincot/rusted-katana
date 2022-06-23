#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use unpack_delicious_sausages::unpack_sausages;

#[bench]
fn bench(bencher: &mut Bencher) {
    let truck = [
        vec!["(----)", "[IIII]", "_HHH_"],
        vec!["IuI", "[))))]", "zz"],
        vec!["[@@@@]", "UwU", "[IlII]"],
    ];
    let truck = black_box(&truck);
    bencher.iter(|| unpack_sausages(truck.to_vec()));
}
