#![feature(test)]

extern crate test;
use guess_the_array::guess;
use test::{black_box, Bencher};

const fn create_array<const N: usize>() -> [i32; N] {
    let mut res = [0; N];
    let mut i = 0;
    while i < res.len() {
        res[i] = i as i32;
        i += 1;
    }
    res
}

const XS1: [i32; 100] = create_array();
const XS2: [i32; 101] = create_array();
const XS3: [i32; 102] = create_array();

#[bench]
fn bench(bencher: &mut Bencher) {
    let f = |xs: &'static [i32]| {
        move |a: usize, b: usize| {
            assert!(
                !(a >= xs.len() || b >= xs.len()),
                "Indices must be within the bounds of 0..n"
            );
            let d = a.abs_diff(b);
            assert!([1, 2].contains(&d), "Indices must differ by 1 or 2");
            xs[a] + xs[b]
        }
    };
    let f1 = f(&XS1);
    let f2 = f(&XS2);
    let f3 = f(&XS3);
    bencher.iter(|| {
        black_box(guess(black_box(f1), black_box(XS1.len())));
        black_box(guess(black_box(f2), black_box(XS2.len())));
        black_box(guess(black_box(f3), black_box(XS3.len())));
    });
}
