#![feature(test)]

extern crate test;
use holiday_iv_leg_room::leg_room;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    const B: &str = "ifbjejaigghkgifbjejaigghkgifbjejaigghkgifbjejaigghkgifbjejaigghkgifbjejaigghkgifbjejaigghkgifbjejaigghkgifbjejaigghkgifbjejaigghkg";

    bencher.iter(|| leg_room(black_box(10000), black_box(B)));
}
