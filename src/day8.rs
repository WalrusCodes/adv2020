use std::collections::HashSet;

type ArgumentType = i32;

#[derive(Debug, Clone)]
enum OpType {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, Clone)]
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

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} {}", self.op_type, self.arg)
    }
}

#[derive(Debug, Clone)]
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
        Handheld {
            instructions,
            ip: 0,
            acc: 0,
            seen: HashSet::new(),
        }
    }

    // Does a step unless about to execute an instruction for the second time or we are at the end
    // of the program.
    fn do_step(self: &mut Self, check_repeat: bool) -> bool {
        use OpType::*;

        if (self.ip < 0) || ((self.ip as usize) >= self.instructions.len()) {
            panic!("IP out of bounds: {}", self.ip);
        }

        if check_repeat && self.seen.contains(&self.ip) {
            return false;
        }

        self.seen.insert(self.ip);

        let op = &self.instructions[self.ip as usize];
        match op.op_type {
            Nop => {}
            Acc => {
                self.acc += op.arg;
            }
            Jmp => {}
        };
        self.ip += match op.op_type {
            Jmp => op.arg,
            _ => 1,
        };

        true
    }

    fn terminated(self: &Self) -> bool {
        (self.ip as usize) == self.instructions.len()
    }

    fn pretty_print_state(self: &Self) -> String {
        if self.terminated() {
            format!("ip: {} acc: {} next: TERMINATED", self.ip, self.acc)
        } else {
            format!(
                "ip: {} acc: {} next: {}",
                self.ip, self.acc, self.instructions[self.ip as usize]
            )
        }
    }

    fn run_until_repeats(self: &mut Self) {
        while self.do_step(true) {
            println!("{}", self.pretty_print_state());
        }
    }

    fn run_until_end(self: &mut Self) -> bool {
        while self.do_step(true) {
            println!("{}", self.pretty_print_state());
            if self.terminated() {
                println!("got to the end :)");
                return true;
            }
        }
        println!("repeated :(");
        false
    }

    fn find_broken_instr(self: &mut Self) -> ArgumentType {
        use OpType::*;

        for (i, op) in self.instructions.iter().enumerate() {
            let new_op = match op.op_type {
                Jmp => Nop,
                Nop => Jmp,
                _ => {
                    continue;
                }
            };
            let mut handheld_tmp = self.clone();
            handheld_tmp.instructions[i].op_type = new_op;
            if handheld_tmp.run_until_end() {
                return handheld_tmp.acc;
            }
        }
        panic!("sad panda");
    }
}

fn main() {
    let contents = std::fs::read_to_string("input/8.txt").expect("read failed");
    let mut hh = Handheld::parse(&contents);
    dbg!(hh.find_broken_instr());
    // dbg!(&hh.ip);
    // hh.run_until_repeats();
    // dbg!(hh.pretty_print_state());
    // hh.run_until_end();
    // println!("{}", hh.pretty_print_state());
}
