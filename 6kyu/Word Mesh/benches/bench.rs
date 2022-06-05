#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

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
    bencher.iter(|| solution::word_mesh(words));
}
