//! <https://www.codewars.com/kata/5265b0885fda8eac5900093b/train/rust>

use core::ops::{Add, Div, Mul, Sub};

use digital::prelude::*;
use unchecked_std::prelude::*;

pub use self::preloaded::{Ast, Operator, Source};

mod preloaded;

impl Operator {
    pub const fn as_fn(self) -> fn(i32, i32) -> i32 {
        match self {
            Self::Add => Add::add,
            Self::Sub => Sub::sub,
            Self::Mul => Mul::mul,
            Self::Div => Div::div,
        }
    }

    const fn precedence(self) -> Precedence {
        match self {
            Self::Add | Self::Sub => Precedence::Additive,
            Self::Mul | Self::Div => Precedence::Multiplicative,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Token<'a> {
    Operator(Operator),
    Value(i32),
    Ident(&'a str),
    ParenOpen,
    ParenClose,
}

impl Token<'_> {
    const fn precedence(&self) -> Option<Precedence> {
        match self {
            &Token::Operator(op) => Some(op.precedence()),
            _ => None,
        }
    }
}

struct Lexer<'a>(&'a str);
type Tokens<'a> = core::iter::Peekable<Lexer<'a>>;

impl<'a> Lexer<'a> {
    /// Move cursor of lexer `n` bytes ahead
    fn eat_bytes(&mut self, n: usize) {
        self.0 = &self.0[n..];
    }

    /// Move cursor of lexer to the first non-whitespace character if there is
    /// one or to the end of string otherwise
    fn eat_whitespace(&mut self) {
        self.eat_bytes(unsafe { self.token_while(|&b| b == b' ') }.len());
    }

    /// Returns a longest prefix of internal string which contains characters
    /// for which predicate `p` holds
    unsafe fn token_while<F: FnMut(&u8) -> bool>(&self, mut p: F) -> &'a str {
        let pos = self
            .0
            .as_bytes()
            .iter()
            .position(|ch| !p(ch))
            .unwrap_or(self.0.len());
        unsafe { self.0.get_unchecked(..pos) }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.eat_whitespace();
        let line = self.0;
        let first = line.as_bytes().first()?;
        let (eaten, token) = match first {
            b'0'..=b'9' => {
                let prefix = unsafe { self.token_while(u8::is_ascii_digit) };
                (prefix.len(), Token::Value(prefix.parse().unwrap()))
            }
            b'a'..=b'z' | b'A'..=b'Z' => {
                let prefix = unsafe { self.token_while(u8::is_ascii_alphabetic) };
                (prefix.len(), Token::Ident(prefix))
            }
            b'+' => (1, Token::Operator(Operator::Add)),
            b'-' => (1, Token::Operator(Operator::Sub)),
            b'*' => (1, Token::Operator(Operator::Mul)),
            b'/' => (1, Token::Operator(Operator::Div)),
            b'(' => (1, Token::ParenOpen),
            b')' => (1, Token::ParenClose),
            _ => panic!(),
        };
        self.eat_bytes(eaten);
        Some(token)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    /// '+' and '-'
    Additive,
    /// of '*' and '/'
    Multiplicative,
}

fn get_args_and_tokens(source: &str) -> (Box<[&str]>, Tokens<'_>) {
    let arg_list_start = source.as_bytes().iter().position(|&b| b == b'[').unwrap() + 1;
    let arg_list_end = source.as_bytes().iter().position(|&b| b == b']').unwrap() + 1;
    let arg_list = unsafe { source.get_unchecked(arg_list_start..arg_list_end - 1) };
    let expr_str = unsafe { source.get_unchecked(arg_list_end..) };
    let args = arg_list.split(' ').filter(|&s| !s.is_empty()).collect();
    let lexer = Lexer(expr_str).peekable();
    (args, lexer)
}

#[derive(Debug, PartialEq, Eq)]
pub enum MyAst {
    /// refers to an argument by its position in argument list
    ArgRef(usize),
    Literal(i32),
    BinOp(Operator, Box<Self>, Box<Self>),
}

impl MyAst {
    /// parse using [Pratt Parser](https://en.wikipedia.org/wiki/Pratt_parser)
    pub fn parse(source: &str) -> Self {
        let (args, mut lexer) = get_args_and_tokens(source);
        Self::parse_expr(&args, &mut lexer, None)
    }

    fn parse_expr<'a>(
        arg_names: &[&'a str],
        tokens: &mut Tokens<'a>,
        prec: Option<Precedence>,
    ) -> Self {
        let current = tokens.next().unwrap();

        let mut expr = match current {
            Token::Value(n) => Self::Literal(n),
            Token::Ident(name) => {
                Self::ArgRef(arg_names.iter().position(|&x| x == name).unwrap() as _)
            }
            Token::ParenOpen => {
                let expr = Self::parse_expr(arg_names, tokens, None);
                assert!(
                    tokens.next() == Some(Token::ParenClose),
                    "no matching paren for open paren found"
                );
                expr
            }
            Token::Operator(_) | Token::ParenClose => panic!(),
        };

        while tokens.peek().and_then(Token::precedence) > prec {
            let token = tokens.next().unwrap();
            expr = if let Token::Operator(op) = token {
                let rhs = Self::parse_expr(arg_names, tokens, Some(op.precedence()));
                Self::BinOp(op, Box::new(expr), Box::new(rhs))
            } else {
                panic!()
            };
        }

        expr
    }
}

impl Ast {
    pub fn parse(source: &str) -> Self {
        let (args, mut lexer) = get_args_and_tokens(source);
        Self::parse_expr(&args[..], &mut lexer, None)
    }

    fn parse_expr<'a>(
        arg_names: &[&'a str],
        tokens: &mut Tokens<'a>,
        prec: Option<Precedence>,
    ) -> Self {
        let current = tokens.next().unwrap();

        let mut expr = match current {
            Token::Value(n) => Self::Value(Source::Imm, n),
            Token::Ident(name) => Self::Value(
                Source::Arg,
                arg_names.iter().position(|&x| x == name).unwrap() as _,
            ),
            Token::ParenOpen => {
                let expr = Self::parse_expr(arg_names, tokens, None);
                assert!(
                    tokens.next() == Some(Token::ParenClose),
                    "no matching paren for open paren found"
                );
                expr
            }
            Token::Operator(_) | Token::ParenClose => panic!(),
        };

        while tokens.peek().and_then(Token::precedence) > prec {
            let token = tokens.next().unwrap();
            expr = if let Token::Operator(op) = token {
                let rhs = Self::parse_expr(arg_names, tokens, Some(op.precedence()));
                Self::BinOp(op, Box::new(expr), Box::new(rhs))
            } else {
                panic!()
            };
        }

        expr
    }
}

fn my_pass2(node: &mut MyAst) {
    let evaluated = match node {
        MyAst::ArgRef(_) | MyAst::Literal(_) => None,
        MyAst::BinOp(op, lhs, rhs) => {
            my_pass2(lhs);
            my_pass2(rhs);
            match (&**lhs, &**rhs) {
                (&MyAst::Literal(x), &MyAst::Literal(y)) => Some(op.as_fn()(x, y)),
                _ => None,
            }
        }
    };
    if let Some(value) = evaluated {
        *node = MyAst::Literal(value);
    }
}

fn my_pass3(node: &MyAst) -> Vec<String> {
    let mut instructions = vec![];
    my_pass3_recursive(&mut instructions, node);
    instructions
}

fn my_pass3_recursive(instructions: &mut Vec<String>, node: &MyAst) {
    match node {
        // NB: we write the values to R0 without taking care about previous value
        // and we don't push values to stack
        &MyAst::Literal(x) => instructions.push(format_string_and_i32("IM ", x)),
        &MyAst::ArgRef(n) => instructions.push(format_string_and_usize("AR ", n)),
        MyAst::BinOp(op, lhs, rhs) => {
            my_pass3_recursive(instructions, lhs);
            // At this point R0 contains the result of evaluating lhs.
            // If rhs is an immediate value (either constant or reference to arg),
            // then we can avoid offloading it to stack.
            let load_code = match **rhs {
                MyAst::Literal(x) => Some(format_string_and_i32("IM ", x)),
                MyAst::ArgRef(n) => Some(format_string_and_usize("AR ", n)),
                MyAst::BinOp(..) => None,
            };
            let (op_code, commutative) = match op {
                Operator::Add => ("AD", true),
                Operator::Sub => ("SU", false),
                Operator::Mul => ("MU", true),
                Operator::Div => ("DI", false),
            };
            if let Some(load_code) = load_code {
                // rhs can be evaluated in-place without touching stack.
                // R0 -> R1
                instructions.push("SW".into());
                // Load rhs to R0
                instructions.push(load_code);
                // Now we have the arguments for op, but in reversed order.
                // For commutative operations we can keep arguments as is.
                // For non-commutative ones we should reverse the order
                if !commutative {
                    instructions.push("SW".into());
                }
            } else {
                // rhs cannot be evaluated in-place.
                // Save R0 to stack
                instructions.push("PU".into());
                // Evaluate rhs
                my_pass3_recursive(instructions, rhs);
                // R0 = value of rhs. R0 -> R1
                instructions.push("SW".into());
                // Restore lhs from stack
                instructions.push("PO".into());
            }
            // Emit instruction for operation
            instructions.push(op_code.into());
        }
    }
}

pub struct Compiler;

impl Compiler {
    pub const fn new() -> Self {
        Self
    }

    pub fn tokenize(&self, program: &str) -> Vec<String> {
        let mut res = Vec::with_capacity(program.len());
        let mut last_space_or_symbol = 0;
        for (i, &c) in program.as_bytes().iter().enumerate() {
            if c < b'0' || b"][".contains(&c) {
                if last_space_or_symbol + 1 < i {
                    if last_space_or_symbol + 2 == i {
                        unsafe {
                            res.push_unchecked(String::from_utf8_unchecked(vec![
                                *program.as_bytes().get_unchecked(last_space_or_symbol + 1),
                            ]));
                        }
                    } else {
                        unsafe {
                            res.push_unchecked(
                                program.get_unchecked(last_space_or_symbol + 1..i).into(),
                            );
                        }
                    }
                }

                if c != b' ' {
                    unsafe { res.push_unchecked(String::from_utf8_unchecked(vec![c])) };
                }

                last_space_or_symbol = i;
            }
        }

        if last_space_or_symbol + 1 < program.len() {
            unsafe { res.push_unchecked(program.get_unchecked(last_space_or_symbol + 1..).into()) };
        }

        res
    }

    pub fn compile(&self, program: &str) -> Vec<String> {
        let mut ast = MyAst::parse(program);
        my_pass2(&mut ast);
        my_pass3(&ast)
    }

    pub fn pass1(&self, program: &str) -> Ast {
        Ast::parse(program)
    }

    #[expect(clippy::self_only_used_in_recursion)]
    pub fn pass2(&self, ast: &Ast) -> Ast {
        match ast {
            &Ast::Value(src, val) => Ast::Value(src, val),
            Ast::BinOp(op, lhs, rhs) => {
                let lhs = self.pass2(lhs);
                let rhs = self.pass2(rhs);
                match (&lhs, &rhs) {
                    (Ast::Value(Source::Imm, lval), Ast::Value(Source::Imm, rval)) => {
                        let new_val = match op {
                            Operator::Add => lval + rval,
                            Operator::Sub => lval - rval,
                            Operator::Mul => lval * rval,
                            Operator::Div => lval / rval,
                        };
                        Ast::Value(Source::Imm, new_val)
                    }
                    _ => Ast::BinOp(*op, Box::new(lhs), Box::new(rhs)),
                }
            }
        }
    }

    pub fn pass3(&self, ast: &Ast) -> Vec<String> {
        let mut instructions = vec![];
        Self::pass3_recursive(&mut instructions, ast);
        instructions
    }

    fn pass3_recursive(instructions: &mut Vec<String>, ast: &Ast) {
        match ast {
            // NB: we write the values to R0 without taking care about previous value
            // and we don't push values to stack
            Ast::Value(op, arg) => {
                instructions.push(match op {
                    Source::Imm => format_string_and_i32("IM ", *arg),
                    Source::Arg => format_string_and_i32("AR ", *arg),
                });
            }
            Ast::BinOp(op, lhs, rhs) => {
                Self::pass3_recursive(instructions, lhs);
                // At this point R0 contains the result of evaluating lhs.
                // If rhs is an immediate value (either constant or reference to arg),
                // then we can avoid offloading it to stack.
                let load_code = match **rhs {
                    Ast::Value(op, arg) => match op {
                        Source::Imm => Some(format_string_and_i32("IM ", arg)),
                        Source::Arg => Some(format_string_and_i32("AR ", arg)),
                    },
                    Ast::BinOp(..) => None,
                };
                let (op_code, commutative) = match op {
                    Operator::Add => ("AD", true),
                    Operator::Sub => ("SU", false),
                    Operator::Mul => ("MU", true),
                    Operator::Div => ("DI", false),
                };
                if let Some(load_code) = load_code {
                    // rhs can be evaluated in-place without touching stack.
                    // R0 -> R1
                    instructions.push("SW".into());
                    // Load rhs to R0
                    instructions.push(load_code);
                    // Now we have the arguments for op, but in reversed order.
                    // For commutative operations we can keep arguments as is.
                    // For non-commutative ones we should reverse the order
                    if !commutative {
                        instructions.push("SW".into());
                    }
                } else {
                    // rhs cannot be evaluated in-place.
                    // Save R0 to stack
                    instructions.push("PU".into());
                    // Evaluate rhs
                    Self::pass3_recursive(instructions, rhs);
                    // R0 = value of rhs. R0 -> R1
                    instructions.push("SW".into());
                    // Restore lhs from stack
                    instructions.push("PO".into());
                }
                // Emit instruction for operation
                instructions.push(op_code.into());
            }
        }
    }
}

fn format_string_and_usize(s: &str, n: usize) -> String {
    let mut res = String::with_capacity(s.len() + usize::MAX_LEN_BASE10);
    unsafe {
        res.push_str_unchecked(s);
        res.write_int_unchecked(n);
    }
    res
}

fn format_string_and_i32(s: &str, n: i32) -> String {
    let mut res = String::with_capacity(s.len() + isize::MAX_LEN_BASE10);
    unsafe {
        res.push_str_unchecked(s);
        res.write_int_unchecked(n);
    }
    res
}
