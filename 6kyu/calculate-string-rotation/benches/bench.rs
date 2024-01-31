#![feature(test)]

extern crate test;
use calculate_string_rotation::shifted_diff;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| shifted_diff(black_box("ckNntxw4xebSYwpQGZacYzwaTZyEc6IEQ8enq2sTYzLjnqAPu0qIiFwpq9m5Mfo9uVphYTVUtNYNvc7grvvWVADd11jTLj4GpiPxrtiqvKfVi4Z6IdGa0ofA0f8SjF4bGqZg9aGLrVSRSJ5J4w3gJoC7JAC1KjLrvBBDoCF2KTELQ9dmqdR5JmPgPC43pfmX78zfCUH6"), black_box("qIiFwpq9m5Mfo9uVphYTVUtNYNvc7grvvWVADd11jTLj4GpiPxrtiqvKfVi4Z6IdGa0ofA0f8SjF4bGqZg9aGLrVSRSJ5J4w3gJoC7JAC1KjLrvBBDoCF2KTELQ9dmqdR5JmPgPC43pfmX78zfCUH6ckNntxw4xebSYwpQGZacYzwaTZyEc6IEQ8enq2sTYzLjnqAPu0")));
}
