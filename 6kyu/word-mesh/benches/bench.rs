#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use word_mesh::word_mesh;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        word_mesh(black_box(&[
            "йаместовн",
            "овнерсхип",
            "хиппоцампус",
            "пусхцарт",
            "цартограпхер",
            "пхеромоне",
        ]))
    });
}
