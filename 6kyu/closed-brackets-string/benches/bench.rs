#![feature(test)]

extern crate test;
use closed_brackets_string::closed_brackets;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| closed_brackets(black_box("(J)JJ((()(J))J(JJ)(((J()JJJJ)J)JJJ(J)(J((J))JJJ))(())((J())((J()()((()(J(JJJ((JJJJJ)(JJ(J((J))(J)JJ))J(()))J))((J))J(J(JJJ()J)))((J()(((JJJJ(J()J())J))JJ))))J()(J)))JJ(J)(J(J(J(JJ))J)(J)((J(J()J)JJJ)))())JJ)))))))()))()J))(())(()))(J)(()JJJ))JJ(J)(J(((J)J(J)")));
}
