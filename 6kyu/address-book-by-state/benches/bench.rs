#![feature(test)]

extern crate test;
use address_book_by_state::by_state;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| by_state(black_box("John Pulsett, 321 King Street, Palmouth MA\nAlisa Gord, 22 Prin Broadway, Georges VA\nOreste Thulas, 11354 East Bridge Road, Pensa OK\nPerry Falpas, 420 Land Road, Beaver Halls PA\nErica Adamson, 200 Station Road, Westbury MA\nPaulo Sims, 8A River Street, Richmond VA\nAnn Wildon, 334 Shore Parkway, Hill View CA\nAl Carpenter, 730 3rd Street, Boston MA")));
}
