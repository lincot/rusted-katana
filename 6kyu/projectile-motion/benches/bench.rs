#![feature(test)]

extern crate test;
use projectile_motion::Projectile;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        let projectile = black_box(Projectile::new(black_box(5), black_box(2), black_box(45)));
        (
            projectile.height_eq(),
            projectile.horiz_eq(),
            projectile.height(0.2),
            projectile.horiz(0.2),
            projectile.landing(),
        )
    });
}
