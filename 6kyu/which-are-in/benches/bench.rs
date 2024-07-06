#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use which_are_in::in_array;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        in_array(
            black_box(&[
                "utate", "us", "lo", "a", "si.", "at", "da", "bi", "a", "Lo", "tell",
            ]),
            black_box(&[
                "a",
                "Lorem",
                "gravida",
                "Proin",
                "massa",
                "vestibulum",
                "sit",
                "elit.",
                "non",
                "nec",
                "mauris",
                "erat",
                "id",
                "ipsum",
            ]),
        )
    });
}
