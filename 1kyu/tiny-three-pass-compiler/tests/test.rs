use tiny_three_pass_compiler::Compiler;

#[test]
fn test_tokenize() {
    for (input, expected) in [
        (
            "[ a b ] a*a + b*b",
            ["[", "a", "b", "]", "a", "*", "a", "+", "b", "*", "b"],
        ),
        (
            "[ first second ] (first + second) / 2",
            [
                "[", "first", "second", "]", "(", "first", "+", "second", ")", "/", "2",
            ],
        ),
    ] {
        assert_eq!(Compiler::new().tokenize(input), expected);
    }
}

const PROGRAM: &str =
    "[ x y z ] (0 - 1) / (0 - 1) * ( 2*3*x + 5*y - 3*z + 600/( z + 0) ) / (1 + 3 - 100+100 + 2*2)";
const ARGV: &[isize] = &[50, 65, 75];
const RES: isize = 51;

#[test]
fn test_compile() {
    let assembly = Compiler::new().compile(PROGRAM);
    assert_eq!(simulate(assembly, ARGV), RES);
}

#[test]
fn test_3_passes() {
    let compiler = Compiler::new();
    let ast = compiler.pass1(PROGRAM);
    let ast = compiler.pass2(&ast);
    let assembly = compiler.pass3(&ast);
    assert_eq!(simulate(assembly, ARGV), RES);
}

fn simulate(assembly: Vec<String>, argv: &[isize]) -> isize {
    let mut r = (0, 0);
    let mut stack = vec![];

    for ins in assembly {
        let mut ws = ins.split_whitespace();
        match ws.next() {
            Some("IM") => r.0 = ws.next().unwrap().parse().unwrap(),
            Some("AR") => r.0 = argv[ws.next().unwrap().parse::<usize>().unwrap()],
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
