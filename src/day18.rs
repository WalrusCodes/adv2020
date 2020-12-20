// 7 * (7 + (4 + 5) + 4)
//                ^
//
// [(7, "*"), (20, "+"),

#[derive(Debug)]
enum Operation {
    Plus,
    Multiply,
}

#[derive(Debug)]
enum Token {
    Number(u32),
    Op(Operation),
    OpenParen,
    CloseParen,
}

#[derive(Debug, Default)]
struct State {
    stack: Vec<(u32, Option<Operation>)>,
}

fn parse(line: &str) -> Vec<Token> {
    use Operation::*;
    use Token::*;

    let mut out = vec![];
    for c in line.chars() {
        match c {
            '0'..='9' => {
                out.push(Number(c.to_digit(10).unwrap()));
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

fn evaluate(expr: &[Token]) -> u32 {
    let mut state = State { stack: vec![(0, Some(Operation::Plus))] };

    todo!();
}

fn main() {
    let contents = std::fs::read_to_string("input/18.txt").expect("read failed");
    let exprs = parse_file(&contents);
    dbg!(&exprs[0]);
}
