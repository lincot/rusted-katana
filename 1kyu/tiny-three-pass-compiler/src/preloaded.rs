#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Source {
    Arg,
    Imm,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Ast {
    BinOp(Operator, Box<Self>, Box<Self>),
    Value(Source, i32),
}

impl Ast {
    pub fn binop(op: Operator, a: Self, b: Self) -> Self {
        Self::BinOp(op, Box::new(a), Box::new(b))
    }
}

#[allow(dead_code)]
pub fn simulate(assembly: &Vec<String>, argv: Vec<i32>) -> i32 {
    let mut r = (0, 0);
    let mut stack: Vec<i32> = vec![];
    let num = |opt: Option<&str>| opt.unwrap().parse::<i32>().unwrap();

    for ins in assembly {
        let mut ws = ins.split_whitespace();
        match ws.next() {
            Some("IM") => r.0 = num(ws.next()),
            Some("AR") => r.0 = argv[num(ws.next()) as usize],
            Some("SW") => r = (r.1, r.0),
            Some("PU") => stack.push(r.0),
            Some("PO") => r.0 = stack.pop().unwrap(),
            Some("AD") => r.0 += r.1,
            Some("SU") => r.0 -= r.1,
            Some("MU") => r.0 *= r.1,
            Some("DI") => r.0 /= r.1,
            _ => panic!("Invalid instruction encountered"),
        }
    }
    r.0
}
