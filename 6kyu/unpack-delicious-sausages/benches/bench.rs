#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use unpack_delicious_sausages::unpack_sausages;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        unpack_sausages(black_box(vec![
            vec!["(----)", "[IIII]", "_HHH_"],
            vec!["IuI", "[))))]", "zz"],
            vec!["[@@@@]", "UwU", "[IlII]"],
        ]))
    });
}
