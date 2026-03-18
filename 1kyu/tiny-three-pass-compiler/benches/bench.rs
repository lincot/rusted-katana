#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use tiny_three_pass_compiler::{
    Ast::{BinOp, Value},
    Compiler,
    Operator::{Add, Div, Mul, Sub},
    Source::{Arg, Imm},
};

#[bench]
fn bench_compile(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(Compiler::new())
            .compile(black_box("[ x y z ] ( 2*3*x + 5*y - 3*z ) / (1 + 3 + 2*2)"))
    });
}

#[bench]
fn bench_tokenize(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(Compiler::new())
            .tokenize(black_box("[ x y z ] ( 2*3*x + 5*y - 3*z ) / (1 + 3 + 2*2)"))
    });
}

#[bench]
fn bench_pass1(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(Compiler::new())
            .pass1(black_box("[ x y z ] ( 2*3*x + 5*y - 3*z ) / (1 + 3 + 2*2)"))
    });
}

#[bench]
// without a in the function name bencher thinks that rusted katana
// implementation is ~5% slower than the identical most upvoted code. it's
// because of the order benchmarks run
fn bench_a_pass2(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(Compiler::new()).pass2(black_box(&BinOp(
            Div,
            Box::new(BinOp(
                Sub,
                Box::new(BinOp(
                    Add,
                    Box::new(BinOp(
                        Mul,
                        Box::new(BinOp(Mul, Box::new(Value(Imm, 2)), Box::new(Value(Imm, 3)))),
                        Box::new(Value(Arg, 0)),
                    )),
                    Box::new(BinOp(Mul, Box::new(Value(Imm, 5)), Box::new(Value(Arg, 1)))),
                )),
                Box::new(BinOp(Mul, Box::new(Value(Imm, 3)), Box::new(Value(Arg, 2)))),
            )),
            Box::new(BinOp(
                Add,
                Box::new(BinOp(
                    Add,
                    Box::new(BinOp(
                        Add,
                        Box::new(BinOp(Add, Box::new(Value(Imm, 1)), Box::new(Value(Imm, 3)))),
                        Box::new(BinOp(Mul, Box::new(Value(Imm, 2)), Box::new(Value(Imm, 2)))),
                    )),
                    Box::new(Value(Imm, 3)),
                )),
                Box::new(BinOp(Mul, Box::new(Value(Imm, 2)), Box::new(Value(Imm, 2)))),
            )),
        )))
    });
}

#[bench]
fn bench_pass3(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(Compiler::new()).pass3(black_box(&BinOp(
            Div,
            Box::new(BinOp(
                Sub,
                Box::new(BinOp(
                    Add,
                    Box::new(BinOp(
                        Mul,
                        Box::new(BinOp(Mul, Box::new(Value(Imm, 2)), Box::new(Value(Imm, 3)))),
                        Box::new(Value(Arg, 0)),
                    )),
                    Box::new(BinOp(Mul, Box::new(Value(Imm, 5)), Box::new(Value(Arg, 1)))),
                )),
                Box::new(BinOp(Mul, Box::new(Value(Imm, 3)), Box::new(Value(Arg, 2)))),
            )),
            Box::new(BinOp(
                Add,
                Box::new(BinOp(
                    Add,
                    Box::new(BinOp(
                        Add,
                        Box::new(BinOp(Add, Box::new(Value(Imm, 1)), Box::new(Value(Imm, 3)))),
                        Box::new(BinOp(Mul, Box::new(Value(Imm, 2)), Box::new(Value(Imm, 2)))),
                    )),
                    Box::new(Value(Imm, 3)),
                )),
                Box::new(BinOp(Mul, Box::new(Value(Imm, 2)), Box::new(Value(Imm, 2)))),
            )),
        )))
    });
}
