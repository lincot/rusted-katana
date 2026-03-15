#![feature(test)]

extern crate test;
use function_1_hello_world::greet;
use test::Bencher;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(greet);
}
