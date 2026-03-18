#![feature(test)]

extern crate test;
use backspaces_in_string::clean_string;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        clean_string(black_box(
            "904rfn#jlkcn#####Djva2###*(#fsdgfd####fsdg###09849###mfenf##dnjn##kmfd;l#mg03###",
        ))
    });
}
