use std::collections::HashMap;

#[derive(Debug)]
enum Cmd<'a> {
    SetMask(&'a str),
    Write(u64, u64),
}

type Program<'a> = Vec<Cmd<'a>>;

#[derive(Debug, Default)]
struct State<'a> {
    memory: HashMap<u64, u64>,
    mask: &'a str,
}

impl<'a> Cmd<'a> {
    fn parse(line: &'a str) -> Cmd<'a> {
        if line.starts_with("mask = ") {
            // parse mask
            // mask = 0XX1XXX1101X101100101001010X1X110000
            Cmd::SetMask(line.split(" = ").nth(1).unwrap())
        } else if line.starts_with("mem[") {
            // parse a memory write
            // mem[41476] = 14032
            let mut parts = line.split(" = ");
            let lhs = parts.next().unwrap();
            let value = parts.next().unwrap().parse().unwrap();
            assert!(lhs.ends_with("]"));
            let address = lhs[4..lhs.len() - 1].parse().unwrap();
            Cmd::Write(address, value)
        } else {
            panic!("invalid line: {}", line);
        }
    }
}

fn parse_input(lines: &str) -> Program {
    lines.lines().map(|line| Cmd::parse(line)).collect()
}

fn apply_mask(value: u64, mask: &str) -> u64 {
    let mut out = 0;
    // mask: 0XX1XXX1101X101100101001010X1X110000
    for (i, c) in mask.chars().rev().enumerate() {
        out |= match c {
            '0' => 0,
            '1' => 1 << i,
            'X' => value & (1 << i),
            _ => {
                panic!("bad char: {}", c);
            }
        };
    }
    out
}

fn part1(program: &Program) -> u64 {
    let mut state = State::default();
    for cmd in program.iter() {
        match cmd {
            Cmd::SetMask(mask) => {
                state.mask = mask;
            }
            &Cmd::Write(address, value) => {
                state.memory.insert(address, apply_mask(value, state.mask));
            }
        }
    }
    state.memory.values().sum()
}

fn calculate_addresses(address: u64, mask: &str, idx: usize, result: &mut Vec<u64>) {
    if idx >= mask.len() {
        result.push(address);
        return;
    }
    let c = mask.chars().rev().nth(idx).unwrap();
    match c {
        '0' => {
            calculate_addresses(address, mask, idx + 1, result);
        }
        '1' => {
            calculate_addresses(address | (1 << idx), mask, idx + 1, result);
        }
        'X' => {
            calculate_addresses(address | (1 << idx), mask, idx + 1, result);
            calculate_addresses(address & !(1 << idx), mask, idx + 1, result);
        }
        _ => {
            panic!("bad char: {}", c);
        }
    };
}

fn part2(program: &Program) -> u64 {
    let mut state = State::default();
    for cmd in program.iter() {
        match cmd {
            Cmd::SetMask(mask) => {
                state.mask = mask;
            }
            &Cmd::Write(address, value) => {
                // 1) calculate all addresses by applying state.mask to address
                let mut floating_addresses = Vec::new();
                calculate_addresses(address, state.mask, 0, &mut floating_addresses);
                // 2) write to them
                for &a in floating_addresses.iter() {
                    // println!("writing {} to {}", value, a);
                    state.memory.insert(a, value);
                }
            }
        }
    }
    state.memory.values().sum()
}

fn main() {
    let contents = std::fs::read_to_string("input/14.txt").expect("read failed");
    let program = dbg!(parse_input(&contents));
    // dbg!(part1(&program));
    dbg!(part2(&program));
}
