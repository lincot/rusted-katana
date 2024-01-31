#![feature(test)]

extern crate test;
use follow_that_spy::find_routes;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        find_routes(black_box(&[
            ["San Policarpo", "Oras"],
            ["Balangiga", "Lawaan"],
            ["Borongan", "Maydolong"],
            ["Jipapad", "Maslog"],
            ["Balangkayan", "Llorente"],
            ["Mercedes", "Guiuan"],
            ["Taft", "Sulat"],
            ["Sulat", "San Julian"],
            ["Arteche", "San Policarpo"],
            ["Oras", "Dolores"],
            ["Dolores", "Can-avid"],
            ["Can-avid", "Taft"],
            ["San Julian", "Borongan"],
            ["Maydolong", "Balangkayan"],
            ["Llorente", "Hernani"],
            ["Hernani", "General MacArthur"],
            ["General MacArthur", "Giporlos"],
            ["Giporlos", "Balangiga"],
            ["Lawaan", "Salcedo"],
            ["Salcedo", "Mercedes"],
            ["Maslog", "Arteche"],
        ]))
    });
}
