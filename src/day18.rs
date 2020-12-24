#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
enum Operation {
    Plus,
    Multiply,
}

#[derive(Debug)]
enum Token {
    Number(u64),
    Op(Operation),
    OpenParen,
    CloseParen,
}

impl Operation {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Plus => a + b,
            Operation::Multiply => a * b,
        }
    }
}

fn parse(line: &str) -> Vec<Token> {
    use Operation::*;
    use Token::*;

    let mut out = vec![];
    for c in line.chars() {
        match c {
            '0'..='9' => {
                out.push(Number(c.to_digit(10).unwrap().into()));
            }
            ' ' => {}
            '+' => {
                out.push(Op(Plus));
            }
            '*' => {
                out.push(Op(Multiply));
            }
            '(' => {
                out.push(OpenParen);
            }
            ')' => {
                out.push(CloseParen);
            }
            _ => {
                panic!("invalid char: {}", c);
            }
        }
    }
    out
}

fn parse_file(lines: &str) -> Vec<Vec<Token>> {
    lines.lines().map(|line| parse(line)).collect()
}

#[derive(Debug, Default)]
struct State {
    stack: Vec<(u64, Option<Operation>)>,
}

impl State {
    fn new() -> Self {
        State {
            stack: vec![(0, Some(Operation::Plus))],
        }
    }
    fn apply(self: &mut Self, token: &Token) {
        use Token::*;
        let mut stack_top = self.stack.last_mut().unwrap();

        match token {
            Number(x) => {
                assert!(stack_top.1.is_some());
                stack_top.0 = stack_top.1.as_ref().unwrap().apply(stack_top.0, *x);
                stack_top.1 = None;
            }
            Op(op) => {
                assert!(stack_top.1.is_none());
                stack_top.1 = Some(*op);
            }
            OpenParen => {
                assert!(stack_top.1.is_some());
                self.stack.push((0, Some(Operation::Plus)));
            }
            CloseParen => {
                assert!(stack_top.1.is_none());
                let num = stack_top.0;
                self.stack.pop();
                self.apply(&Number(num));
            }
        }
    }
}

fn evaluate(expr: &[Token]) -> u64 {
    let mut state = State::new();
    for token in expr {
        state.apply(token);
    }
    // dbg!(&state.stack);
    assert_eq!(state.stack.len(), 1);
    assert!(state.stack[0].1.is_none());
    let result = state.stack[0].0;
    dbg!(result)
}

fn main() {
    let contents = std::fs::read_to_string("input/18.txt").expect("read failed");
    let exprs = parse_file(&contents);
    // dbg!(&exprs[0]);
    // let result = evaluate(&exprs[0]);
    let result: u64 = exprs.iter().map(|x| evaluate(x)).sum();
    dbg!(&result);
}
