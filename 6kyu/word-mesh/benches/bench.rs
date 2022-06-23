#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use word_mesh::word_mesh;

#[bench]
fn bench(bencher: &mut Bencher) {
    let words = black_box(&[
        "йаместовн",
        "овнерсхип",
        "хиппоцампус",
        "пусхцарт",
        "цартограпхер",
        "пхеромоне",
    ]);
    bencher.iter(|| word_mesh(words));
}
