#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use tiny_three_pass_compiler::{
    self,
    Ast::{BinOp, UnOp},
    Compiler,
};

#[bench]
fn bench_compile(bencher: &mut Bencher) {
    let program = black_box("[ x y z ] ( 2*3*x + 5*y - 3*z ) / (1 + 3 + 2*2)");
    let mut compiler = Compiler::new();
    bencher.iter(|| {
        black_box(compiler.compile(program));
    });
}

#[bench]
fn bench_tokenize(bencher: &mut Bencher) {
    let program = black_box("[ x y z ] ( 2*3*x + 5*y - 3*z ) / (1 + 3 + 2*2)");
    let compiler = Compiler::new();
    bencher.iter(|| {
        black_box(compiler.tokenize(program));
    });
}

#[bench]
fn bench_pass1(bencher: &mut Bencher) {
    let program = black_box("[ x y z ] ( 2*3*x + 5*y - 3*z ) / (1 + 3 + 2*2)");
    let mut compiler = Compiler::new();
    bencher.iter(|| {
        black_box(compiler.pass1(program));
    });
}

#[bench]
fn bench_pass2(bencher: &mut Bencher) {
    let ast = black_box(BinOp(
        "/".into(),
        Box::new(BinOp(
            "-".into(),
            Box::new(BinOp(
                "+".into(),
                Box::new(BinOp(
                    "*".into(),
                    Box::new(BinOp(
                        "*".into(),
                        Box::new(UnOp("imm".into(), 2)),
                        Box::new(UnOp("imm".into(), 3)),
                    )),
                    Box::new(UnOp("arg".into(), 0)),
                )),
                Box::new(BinOp(
                    "*".into(),
                    Box::new(UnOp("imm".into(), 5)),
                    Box::new(UnOp("arg".into(), 1)),
                )),
            )),
            Box::new(BinOp(
                "*".into(),
                Box::new(UnOp("imm".into(), 3)),
                Box::new(UnOp("arg".into(), 2)),
            )),
        )),
        Box::new(BinOp(
            "+".into(),
            Box::new(BinOp(
                "+".into(),
                Box::new(UnOp("imm".into(), 1)),
                Box::new(UnOp("imm".into(), 3)),
            )),
            Box::new(BinOp(
                "*".into(),
                Box::new(UnOp("imm".into(), 2)),
                Box::new(UnOp("imm".into(), 2)),
            )),
        )),
    ));
    let ast = black_box(&ast);
    let mut compiler = Compiler::new();
    bencher.iter(|| {
        black_box(compiler.pass2(ast));
    });
}

#[bench]
fn bench_pass3(bencher: &mut Bencher) {
    let ast = black_box(BinOp(
        "/".into(),
        Box::new(BinOp(
            "-".into(),
            Box::new(BinOp(
                "+".into(),
                Box::new(BinOp(
                    "*".into(),
                    Box::new(BinOp(
                        "*".into(),
                        Box::new(UnOp("imm".into(), 2)),
                        Box::new(UnOp("imm".into(), 3)),
                    )),
                    Box::new(UnOp("arg".into(), 0)),
                )),
                Box::new(BinOp(
                    "*".into(),
                    Box::new(UnOp("imm".into(), 5)),
                    Box::new(UnOp("arg".into(), 1)),
                )),
            )),
            Box::new(BinOp(
                "*".into(),
                Box::new(UnOp("imm".into(), 3)),
                Box::new(UnOp("arg".into(), 2)),
            )),
        )),
        Box::new(BinOp(
            "+".into(),
            Box::new(BinOp(
                "+".into(),
                Box::new(UnOp("imm".into(), 1)),
                Box::new(UnOp("imm".into(), 3)),
            )),
            Box::new(BinOp(
                "*".into(),
                Box::new(UnOp("imm".into(), 2)),
                Box::new(UnOp("imm".into(), 2)),
            )),
        )),
    ));
    let ast = black_box(&ast);
    let mut compiler = Compiler::new();
    bencher.iter(|| {
        black_box(compiler.pass3(ast));
    });
}
