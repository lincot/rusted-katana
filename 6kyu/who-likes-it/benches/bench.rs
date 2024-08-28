#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use who_likes_it::likes;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(likes(black_box(&[])));
        black_box(likes(black_box(&["Peter"])));
        black_box(likes(black_box(&["Jacob", "Alex"])));
        black_box(likes(black_box(&["Max", "John", "Mark"])));
        black_box(likes(black_box(&["Alex", "Jacob", "Mark", "Max"])));
    });
}
