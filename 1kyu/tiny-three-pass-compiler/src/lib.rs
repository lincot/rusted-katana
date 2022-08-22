//! <https://www.codewars.com/kata/5265b0885fda8eac5900093b/train/rust>

// TODO: this solution is WIP

use std::ops::{Add, Div, Mul, Sub};

mod parse {
    use std::ops::{Add, Div, Mul, Sub};

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum BinOp {
        Plus,
        Minus,
        Mult,
        Div,
    }

    impl BinOp {
        pub fn as_string(self) -> String {
            match self {
                Self::Plus => "+",
                Self::Minus => "-",
                Self::Mult => "*",
                Self::Div => "/",
            }
            .into()
        }

        pub fn as_fn(self) -> fn(u64, u64) -> u64 {
            match self {
                Self::Plus => Add::add,
                Self::Minus => Sub::sub,
                Self::Mult => Mul::mul,
                Self::Div => Div::div,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Token<'a> {
        BinOp(BinOp),
        Value(u64),
        Ident(&'a str),
        ParenOpen,
        ParenClose,
    }

    struct Lexer<'a>(&'a str);
    type Tokens<'a> = ::std::iter::Peekable<Lexer<'a>>;

    impl<'a> Lexer<'a> {
        // Move cursor of lexer `n` bytes ahead
        fn eat_bytes(&mut self, n: usize) {
            self.0 = &self.0[n..];
        }

        // Move cursor of lexer to the first non-whitespace character if there is one
        // or to the end of string otherwise
        fn eat_whitespace(&mut self) {
            let pos = self
                .0
                .find(|ch: char| !ch.is_whitespace())
                .unwrap_or(self.0.len());
            self.eat_bytes(pos);
        }

        // Returns a longest prefix of internal string which contains characters
        // for which predicate `p` holds
        fn token_while<F: FnMut(char) -> bool>(&self, mut p: F) -> &'a str {
            let pos = self.0.find(|ch| !p(ch)).unwrap_or(self.0.len());
            &self.0[..pos]
        }
    }

    impl<'a> Iterator for Lexer<'a> {
        type Item = Token<'a>;

        fn next(&mut self) -> Option<Self::Item> {
            self.eat_whitespace();
            let line = self.0;
            //let first = line.chars().next()?; // requires a newer version of Rust
            let first = match line.chars().next() {
                Some(ch) => ch,
                None => return None,
            };
            let (eaten, token) = match first {
                '0'..='9' => {
                    //let prefix = self.token_while(char::is_ascii_digit);
                    let prefix = self.token_while(|ch| ('0'..='9').contains(&ch));
                    (
                        prefix.len(),
                        Token::Value(prefix.parse().expect("parse error")),
                    )
                }
                'a'..='z' | 'A'..='Z' => {
                    let prefix = self.token_while(char::is_alphabetic);
                    (prefix.len(), Token::Ident(prefix))
                }
                '+' => (1, Token::BinOp(BinOp::Plus)),
                '-' => (1, Token::BinOp(BinOp::Minus)),
                '*' => (1, Token::BinOp(BinOp::Mult)),
                '/' => (1, Token::BinOp(BinOp::Div)),
                '(' => (1, Token::ParenOpen),
                ')' => (1, Token::ParenClose),
                _ => panic!(),
            };
            self.eat_bytes(eaten);
            Some(token)
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum Ast {
        ArgRef(u8), //refers to an argument by it's position in argument list
        Literal(u64),
        BinOp(BinOp, Box<Self>, Box<Self>),
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    enum Precedence {
        Additive,       // of '+' and '-'
        Multiplicative, // of '*' and '/'
    }

    impl BinOp {
        const fn precedence(self) -> Precedence {
            match self {
                Self::Plus | Self::Minus => Precedence::Additive,
                Self::Mult | Self::Div => Precedence::Multiplicative,
            }
        }
    }

    impl<'a> Token<'a> {
        const fn precedence(&self) -> Option<Precedence> {
            match self {
                &Token::ParenOpen | &Token::ParenClose | &Token::Ident(_) | &Token::Value(_) => {
                    None
                }
                &Token::BinOp(op) => Some(op.precedence()),
            }
        }
    }

    // AST is parsed using [Pratt Parser](https://en.wikipedia.org/wiki/Pratt_parser)
    impl Ast {
        pub fn parse(source: &str) -> Self {
            let arg_list_end = source.find(']').unwrap() + 1;
            let (arg_list, expr_str) = source.split_at(arg_list_end);
            let args = arg_list
                .trim_matches(&['[', ']'][..]) //get rid of surrounding brackets
                .split(' ')
                .filter(|&s| !s.is_empty())
                .collect::<Vec<_>>();
            let mut lexer = Lexer(expr_str).peekable();
            Self::parse_expr(&args[..], &mut lexer, None)
        }

        fn parse_expr<'a>(
            arg_names: &[&'a str],
            tokens: &mut Tokens<'a>,
            prec: Option<Precedence>,
        ) -> Self {
            let current = tokens.next().expect("no tokens to parse");

            let mut expr = match current {
                Token::Value(n) => Self::Literal(n),
                Token::Ident(name) => {
                    Self::ArgRef(arg_names.iter().position(|&x| x == name).unwrap() as _)
                }
                Token::ParenOpen => {
                    let expr = Self::parse_expr(arg_names, tokens, None);
                    assert_eq!(
                        tokens.next(),
                        Some(Token::ParenClose),
                        "no matching paren for open paren found"
                    );
                    expr
                }
                Token::BinOp(_) | Token::ParenClose => panic!(),
            };

            while tokens.peek().and_then(Token::precedence) > prec {
                let token = tokens.next().unwrap();
                expr = match token {
                    Token::BinOp(op) => Self::parse_infix(arg_names, expr, tokens, op),
                    _ => panic!(),
                };
            }

            expr
        }

        fn parse_infix<'a>(
            arg_names: &[&'a str],
            expr: Self,
            tokens: &mut Tokens<'a>,
            op: BinOp,
        ) -> Self {
            let prec = Some(op.precedence());
            let rhs = Self::parse_expr(arg_names, tokens, prec);
            Self::BinOp(op, Box::new(expr), Box::new(rhs))
        }
    }
}

#[derive(Debug, Clone)]
pub enum Ast {
    BinOp(String, Box<Self>, Box<Self>),
    UnOp(String, u64),
}

use parse::Ast as MyAst;

fn to_dumb_ast(ast: MyAst) -> Ast {
    match ast {
        MyAst::Literal(n) => Ast::UnOp("imm".into(), n),
        MyAst::ArgRef(n) => Ast::UnOp("arg".into(), n as _),
        MyAst::BinOp(op, lhs, rhs) => Ast::BinOp(
            op.as_string(),
            Box::new(to_dumb_ast(*lhs)),
            Box::new(to_dumb_ast(*rhs)),
        ),
    }
}

fn fold_constants(node: &mut MyAst) {
    let evaluated = match *node {
        MyAst::ArgRef(_) | MyAst::Literal(_) => None,
        MyAst::BinOp(ref op, ref mut lhs, ref mut rhs) => {
            fold_constants(lhs);
            fold_constants(rhs);
            let args = match (&**lhs, &**rhs) {
                (&MyAst::Literal(x), &MyAst::Literal(y)) => Some((x, y)),
                _ => None,
            };
            args.map(|(x, y)| op.as_fn()(x, y))
        }
    };
    if let Some(value) = evaluated {
        *node = MyAst::Literal(value);
    }
}

fn move_immediate_to_rhs(node: &mut MyAst) {
    match *node {
        MyAst::Literal(_) | MyAst::ArgRef(_) => (),
        MyAst::BinOp(_op, ref mut lhs, ref mut rhs) => {
            let lhs_is_immediate = match **lhs {
                MyAst::Literal(_) | MyAst::ArgRef(_) => true,
                MyAst::BinOp(..) => {
                    move_immediate_to_rhs(lhs);
                    false
                }
            };
            let rhs_is_immediate = match **rhs {
                MyAst::Literal(_) | MyAst::ArgRef(_) => true,
                MyAst::BinOp(..) => {
                    move_immediate_to_rhs(rhs);
                    false
                }
            };
            if lhs_is_immediate && !rhs_is_immediate {
                ::std::mem::swap(lhs, rhs);
            }
        }
    }
}

// Compiler::pass3 but on MyAst
fn emit_instructions(node: &MyAst) -> Vec<String> {
    let mut instructions = vec![];
    emit_instructions_recursive(&mut instructions, node);
    instructions
}

// Compiler::pass3_recursive but more effective
// since this implementation avoids excessive operations with stack
fn emit_instructions_recursive(instructions: &mut Vec<String>, node: &MyAst) {
    match *node {
        // NB: we write the values to R0 without taking care about previous value
        // and we don't push values to stack
        MyAst::Literal(x) => instructions.push(format!("IM {}", x)),
        MyAst::ArgRef(n) => instructions.push(format!("AR {}", n)),
        MyAst::BinOp(op, ref lhs, ref rhs) => {
            use parse::BinOp;

            emit_instructions_recursive(instructions, lhs);
            // At this point R0 contains the result of evaluating lhs.
            // If rhs is an immediate value (either constant or reference to arg),
            // then we can avoid offloading it to stack.
            let load_code = match **rhs {
                MyAst::Literal(x) => Some(format!("IM {}", x)),
                MyAst::ArgRef(n) => Some(format!("AR {}", n)),
                MyAst::BinOp(..) => None,
            };
            let (op_code, commutative) = match op {
                BinOp::Plus => ("AD", true),
                BinOp::Minus => ("SU", false),
                BinOp::Mult => ("MU", true),
                BinOp::Div => ("DI", false),
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
                instructions.push("PO".into());
                // Evaluate rhs
                emit_instructions_recursive(instructions, rhs);
                // R0 = value of rhs. R0 -> R1:
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
        let program = program.as_bytes();

        let mut res = Vec::with_capacity(program.len() * 2 / 3);

        let mut last_space_or_symbol = 0;

        for (i, &c) in program.iter().enumerate() {
            if c < b'0' || [b']', b'['].contains(&c) {
                if last_space_or_symbol + 1 < i {
                    res.push(unsafe {
                        String::from_utf8_unchecked(
                            program.get_unchecked(last_space_or_symbol + 1..i).to_vec(),
                        )
                    });
                }

                if c != b' ' {
                    res.push(unsafe { String::from_utf8_unchecked(vec![c]) });
                }

                last_space_or_symbol = i;
            }
        }

        res
    }

    pub fn compile(&mut self, program: &str) -> Vec<String> {
        let mut ast = MyAst::parse(program);
        fold_constants(&mut ast);
        move_immediate_to_rhs(&mut ast);
        emit_instructions(&ast)
    }

    pub fn pass1(&mut self, program: &str) -> Ast {
        to_dumb_ast(MyAst::parse(program))
    }

    pub fn pass2(&mut self, ast: &Ast) -> Ast {
        match *ast {
            Ast::UnOp(..) => ast.clone(),
            Ast::BinOp(ref op, ref lhs, ref rhs) => {
                let lhs = self.pass2(lhs);
                let rhs = self.pass2(rhs);
                match (&lhs, &rhs) {
                    (&Ast::UnOp(ref lop, lval), &Ast::UnOp(ref rop, rval)) => {
                        if lop != "imm" || rop != "imm" {
                            return Ast::BinOp(
                                op.clone(),
                                Box::new(lhs.clone()),
                                Box::new(rhs.clone()),
                            );
                        }
                        let func: fn(u64, u64) -> u64 = match op.as_str() {
                            "+" => Add::add,
                            "-" => Sub::sub,
                            "*" => Mul::mul,
                            "/" => Div::div,
                            _ => panic!(),
                        };
                        let new_val = func(lval, rval);
                        Ast::UnOp(lop.clone(), new_val)
                    }
                    _ => Ast::BinOp(op.clone(), Box::new(lhs.clone()), Box::new(rhs.clone())),
                }
            }
        }
    }

    pub fn pass3(&mut self, ast: &Ast) -> Vec<String> {
        let mut instructions = vec![];
        Self::pass3_recursive(&mut instructions, ast);
        instructions
    }

    fn pass3_recursive(instructions: &mut Vec<String>, ast: &Ast) {
        match *ast {
            Ast::UnOp(ref op, arg) => {
                let introduction = match op.as_str() {
                    "imm" => format!("IM {}", arg),
                    "arg" => format!("AR {}", arg),
                    _ => panic!(),
                };
                instructions.push(introduction);
                instructions.push("PU".into());
            }
            Ast::BinOp(ref op, ref lhs, ref rhs) => {
                Self::pass3_recursive(instructions, lhs);
                Self::pass3_recursive(instructions, rhs);

                instructions.push("PO".into());
                instructions.push("SW".into());

                instructions.push("PO".into());

                instructions.push(
                    (match op.as_str() {
                        "+" => "AD",
                        "-" => "SU",
                        "*" => "MU",
                        "/" => "DI",
                        _ => panic!(),
                    })
                    .into(),
                );

                instructions.push("PU".into());
            }
        }
    }
}
