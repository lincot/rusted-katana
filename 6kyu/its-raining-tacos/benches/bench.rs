#![feature(test)]

extern crate test;
use its_raining_tacos::rain_tacos;
use test::{black_box, Bencher};

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| {
        for landscape in [
            "     \n     \nDDDDD",
            "DDDDD\nDDDDD\nDDDDD",
            "       \n       \n   D   \n  DDD  \n TACOS ",
            "* *\n* *\n* *\n* *\n* *\n* *\n* *\n* *\n* *\n* *",
            "          \n    ==    \n          \n          \n          ",
        ] {
            black_box(rain_tacos(black_box(landscape)));
        }
    });
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| {
        for landscape in [
            "     \n     \nДДДДД",
            "ДДДДД\nДДДДД\nДДДДД",
            "       \n       \n   Д   \n  ДДД  \n ТАКОС ",
            "≈ ≈\n≈ ≈\n≈ ≈\n≈ ≈\n≈ ≈\n≈ ≈\n≈ ≈\n≈ ≈\n≈ ≈\n≈ ≈",
            "          \n    ≈≈    \n          \n          \n          ",
        ] {
            black_box(rain_tacos(black_box(landscape)));
        }
    });
}
