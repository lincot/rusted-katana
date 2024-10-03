#![feature(test)]

extern crate test;
use dead_ants::dead_ant_count;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    const ANTS: &str = "...ant...ant..nat.ant.t..ant...ant..ant..ant.anant..t...ant...ant..nat.ant.t..ant...ant..ant..ant.anant..t...ant...ant..nat.ant.t..ant...ant..ant..ant.anant..t...ant...ant..nat.ant.t..ant...ant..ant..ant.anant..t...ant...ant..nat.ant.t..ant...ant..ant..ant.anant..tant ant .... a ntant ant .... a ntant ant .... a ntant ant .... a nt";

    bencher.iter(|| dead_ant_count(black_box(ANTS)));
}
