#![feature(test)]

extern crate test;
use approximate_solution_of_differential_equation_with_runge_kutta_4::rk4;
use test::{Bencher, black_box};

#[bench]
#[allow(clippy::suboptimal_flops)]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        rk4(
            black_box(0.),
            black_box(1.),
            black_box(0.1),
            black_box(|x, y| (5. * x * x - y) / (x + y).exp()),
            black_box(1.),
        )
    });
}
