#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use unpack_delicious_sausages::unpack_sausages;

#[bench]
fn bench(bencher: &mut Bencher) {
    let truck = black_box(vec![
        vec!["(----)", "[IIII]", "_HHH_"],
        vec!["IuI", "[))))]", "zz"],
        vec!["[@@@@]", "UwU", "[IlII]"],
    ]);
    bencher.iter(|| unpack_sausages(truck.clone()));
}
