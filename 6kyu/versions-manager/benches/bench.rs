#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use versions_manager::VersionManager;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        let vm = &mut VersionManager::from_version(black_box("1.2.3.4")).unwrap();
        let vm = black_box(vm).major();
        let vm = black_box(vm).minor();
        let vm = black_box(vm).patch();
        let vm = black_box(vm).rollback().unwrap();
        black_box(vm).release()
    });
}
