#![feature(test)]

extern crate test;
use holiday_iii_fire_on_the_boat::fire_fight;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        fire_fight(black_box(
            "Fire Deck Engine Sail Deck Fire Fire Fire Rudder Fire Boat Fire Fire Captain",
        ))
    });
}
