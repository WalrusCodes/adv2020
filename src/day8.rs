use std::collections::HashSet;

type ArgumentType = i32;

#[derive(Debug)]
enum OpType {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug)]
struct Op {
    op_type: OpType,
    arg: ArgumentType,
}

impl Op {
    // Parses a string like "acc +17" into an Op.
    fn parse(line: &str) -> Op {
        let (cmd, arg_str) = line.split_at(4);
        let op_type = match cmd.trim() {
            "nop" => OpType::Nop,
            "acc" => OpType::Acc,
            "jmp" => OpType::Jmp,
            _ => {
                panic!("invalid cmd {}", cmd);
            }
        };
        let arg = arg_str.parse::<ArgumentType>().unwrap();
        Op { op_type, arg }
    }
}

#[derive(Debug)]
struct Handheld {
    instructions: Vec<Op>,
    ip: ArgumentType,
    acc: ArgumentType,
    seen: HashSet<ArgumentType>,
}

impl Handheld {
    fn parse(lines: &str) -> Handheld {
        let instructions = lines
            .lines()
            .map(|line| Op::parse(line))
            .collect::<Vec<Op>>();
        Handheld { instructions, ip: 0, acc: 0, seen: HashSet::new() }
    }

    // Does a step unless about to execute an instruction for the second time.
    fn do_step(self: &mut Self) -> bool {
        use OpType::*;

        if (self.ip < 0) || ((self.ip as usize) >= self.instructions.len()) {
            panic!("IP out of bounds: {}", self.ip);
        }

        if self.seen.contains(&self.ip) {
            return false;
        }

        self.seen.insert(self.ip);

        let op = &self.instructions[self.ip as usize];
        match op.op_type {
            Nop => {},
            Acc => { self.acc += op.arg; },
            Jmp => { },
        };
        self.ip += match op.op_type {
            Jmp => op.arg,
            _ => 1,
        };
        
        true
    }

    fn run(self: &mut Self) {
        while self.do_step() {}
    }
}

fn main() {
    let contents = std::fs::read_to_string("input/8.txt").expect("read failed");
    let mut hh = Handheld::parse(&contents);
    dbg!(&hh.ip);
    hh.run();
    dbg!(&hh.ip);
    dbg!(&hh.acc);
}
