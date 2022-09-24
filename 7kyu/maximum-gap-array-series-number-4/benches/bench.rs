#![no_std]
#![feature(test)]

extern crate test;
use maximum_gap_array_series_number_4::max_gap;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let xs = black_box(&[
        455, -75, -387, -101, 278, 143, -418, 64, -478, -323, -62, 302, -172, 470, -440, -340, 341,
        -218, 115, 353, 7, 38, 159, -281, -221, -421, -424, 483, 248, -219, -194, -23, -201, 77,
        -54, 110, -125, -79, 353, 461, -175, -283, -345, 3, 411, 131, 222, -320, 264, -67, -280,
        -349, -401, -11, -198, -32, 125, -452, -327, -252, 231, 170, 121, -191, 248, 235, 383, -90,
        -405, -331, 180, 248, 367, -238, 276, 195, 477, -349, -79, -338, 204, 162, -112, -161,
        -411, -454, 264, -191, 107, 271, -278, -23, -303, 280, -168, 231, -96, 51, 102, 300, -123,
        -168, -101, -182, 475, -204, -117, 177, -66, 29, 75, -76, -108, 379, -237, -382, 437, 400,
        -182, -175, 245, 310, 165, 273, 365, 174, -80, 95, -19, -140, -386, 67, -303, -478, 298,
        251, 451, 267, -215, -330, 181, 84, 167, 136, 317, 492, 470, -498, 232, 341, 155, -145,
        -86, -357, 382, 349, -479, -108, 20, 50, -338, 455, -389, -314, -121, -314, -457, -346,
        -288, 97, 26, 337, 97, 154, -454, -279, 431, -237, -243, -454, -136, -129, -9, -337, 222,
        427, 151, 225, 301, 228, -382, 1, -309, -426, 130, 126, 108, -401, 465, 306, -143, -111,
        448, -237, -16, 221, -266, -188, 425, 11, 286, 75, 352, -63, -279, -456, -24, -404, -403,
        205, 324, 340, -167, -263, -290, 373, -318, -82, -171, -24, -371, 293, -24, -103, 335,
        -125, 351, 31, -337, 96, 399, -179, -232, -50, 404, 186, -407, 488, 213, -392, 254, -149,
        -375, -73, 41,
    ]);
    bencher.iter(|| max_gap(xs));
}
