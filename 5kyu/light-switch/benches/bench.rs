#![feature(test)]

extern crate test;
use light_switch::light_switch;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let corresponding_lights_list = vec![
        vec![0, 2, 4, 5, 6, 9, 10],
        vec![1, 3, 5, 6, 7, 8, 11],
        vec![1, 2, 3, 4, 6, 7, 8, 11],
        vec![2, 4, 9],
        vec![7, 8, 9, 10],
        vec![1, 4, 8, 11],
        vec![6, 9],
        vec![8, 9, 10],
        vec![2, 3, 5, 7, 10, 11],
        vec![
            12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
        ],
        vec![12, 13, 14, 15, 16, 17, 18, 19],
        vec![21, 22, 23, 24, 25, 26, 27, 28, 29],
        vec![
            4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
            27, 28, 29, 30,
        ],
        vec![6, 7],
        vec![17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28],
    ];
    bencher.iter(|| light_switch(black_box(31), black_box(&corresponding_lights_list)));
}
