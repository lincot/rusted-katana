#![feature(test)]

extern crate test;
use holiday_iii_fire_on_the_boat::fire_fight;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s =
        black_box("Fire Deck Engine Sail Deck Fire Fire Fire Rudder Fire Boat Fire Fire Captain");
    bencher.iter(|| fire_fight(s));
}
