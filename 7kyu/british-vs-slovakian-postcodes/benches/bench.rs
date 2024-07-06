#![feature(test)]

extern crate test;
use british_vs_slovakian_postcodes::which_postcode;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(which_postcode(black_box("G4 7AH")));
        black_box(which_postcode(black_box("G12 8NU")));
        black_box(which_postcode(black_box("dN1 1AA")));
        black_box(which_postcode(black_box("Se21 7AA")));
        black_box(which_postcode(black_box("G4 7Ah  ")));

        black_box(which_postcode(black_box("040 01")));
        black_box(which_postcode(black_box("070 08")));
        black_box(which_postcode(black_box("  810 08")));

        black_box(which_postcode(black_box("G4  7AH")));
        black_box(which_postcode(black_box("12 8NU")));
        black_box(which_postcode(black_box("DN1 AAA")));
        black_box(which_postcode(black_box("SE21 AA7")));
        black_box(which_postcode(black_box("G47AH")));
        black_box(which_postcode(black_box("04001")));
    });
}
