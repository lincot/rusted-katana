#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use the_latest_clock::latest_clock;

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut v = Vec::with_capacity(24073);
    for h in 0..if cfg!(miri) { 5 } else { 24 } {
        for m in 0..if cfg!(miri) { 5 } else { 60 } {
            let mut arr = [h / 10, h % 10, m / 10, m % 10];
            arr.sort_unstable();
            loop {
                v.push(arr);
                if !next_permutation(&mut arr) {
                    break;
                }
            }
        }
    }

    bencher.iter(|| {
        for &[a, b, c, d] in &v {
            black_box(latest_clock(
                black_box(a),
                black_box(b),
                black_box(c),
                black_box(d),
            ));
        }
    });
}

fn next_permutation(arr: &mut [u8]) -> bool {
    let mut i = arr.len() - 1;
    while i > 0 && arr[i - 1] >= arr[i] {
        i -= 1;
    }

    if i == 0 {
        return false;
    }

    let mut j = arr.len() - 1;
    while *unsafe { arr.get_unchecked(j) } <= arr[i - 1] {
        j -= 1;
    }

    arr.swap(i - 1, j);

    arr[i..].reverse();

    true
}
