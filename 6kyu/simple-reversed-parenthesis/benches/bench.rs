#![feature(test)]

extern crate test;
use simple_reversed_parenthesis::solve;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| solve(black_box("))(())))())))((())()(()((()())))))))))(())))()((()()))))())((((()))()))()))))(())(()))((()()(((())())((())(()))()(()()))()(())()()())()))(()((()()(((((((((((()())()(()())()(())))(((()()))()))))(()(((()))(()))(((())()()((((()()((())(()()))())((()(())))(()))())()(())((((())))()))()))(())((()()()())())()))))()))))())()))())()))))))(((()(()))()))()((()(((((((())()((()()()())())()(((((((()((()))))())())))))(())()))((()((()())(())())))((())((())(()))))(()()())))(())))((((()())(((())())()(())((()()()(()()))()(())((()())()))))))))((((())))()())()))))(()()()))())()))(()((())()))))))(()))((()))((()())(((((()()()))()(((((())))()))(())())(()(((())))())))))))((())()()))))()()(()()()((((()((()())())()(())))(()))((((((()())()")));
}
