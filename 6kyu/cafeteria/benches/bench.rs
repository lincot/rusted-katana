#![feature(test)]

extern crate test;
use cafeteria::CoffeeBuilder;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(
            black_box(CoffeeBuilder::new())
                .set_black_coffee()
                .with_sugar(black_box("Regular".into()))
                .with_milk(black_box(3.2))
                .build(),
        );
        black_box(black_box(CoffeeBuilder::new()).set_cubano_coffee());
        black_box(black_box(CoffeeBuilder::new()).set_antoccino_coffee());
    });
}
