#![no_std]
#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use whats_up_next::next_item;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(next_item(
                black_box(&[
                    "iWWbOIKyn87NEGC6MlkIDlDVhZXmHU",
                    "qL1X3HxjblsmtNM6SnFThr7q6vhZ7p",
                    "TfDVVypMthsDrANmviOTDX75c2LfM0",
                    "HiHSCiqdxn11o6aaNd9PAuNx4G1Jd6",
                    "94SQZ4Z56Dg6PiHGhvTE5g1YIqVnEj",
                    "m9AWBsOkLCwJEuj6RFTkC1DeUZg2VZ",
                    "ekKEeN1QdAU8XaQjrGl5OfL9CDx6GO",
                    "ri41joVQS9Qqqx5gB7N4KhpMY2D4xP",
                    "QlElqOt7FWWcypKu5a29tCIuVVDOca",
                    "9Rdhn4EQNQew8hpU0lv3gKfQk8ASs9",
                    "6hNdsqfiaQ3BMgKz4k8shqPgVyVbEB",
                    "e45hA0tGdl8cUXg6OUdXOffU4Qw6tJ",
                    "brLRk64xRv5VpBEFm7WbcxPT6ld7vk",
                    "fJdCGhSmXvF6Q05eMjGbKO2gUYm2EV",
                    "LSFP9QKqTcmvQ4SSTSjXJJHlrMQSTA",
                    "xLHfCfaaRhob6GdW4EYknK1Do35JPu",
                    "IRIrqWx4hRhW3X8OVdyV9cW7QB72Pe",
                    "W8KiBxUreSiHhPov6sfsV2ovoKWCAK",
                    "7QaDrWKrTVABvZbAdTqlkhNkQkXy0m",
                    "0143TwUyYfTDhLajJxtZWSBFSdaBCs",
                ]),
                black_box("9Rdhn4EQNQew8hpU0lv3gKfQk8ASs9"),
            ));
        }
    });
}
