// Input patterns:
// [NSEW]\d+
// L\d+
// R\d+
// F\d+

#[derive(Debug)]
struct Cmd {
    cmd: char,
    arg: u32,
}

#[derive(Debug)]
struct State {
    // +x is east
    x: i32,
    // +y is north
    y: i32,
    // Degrees clockwise from north.
    dir: i32,
}

impl Cmd {
    fn parse(line: &str) -> Cmd {
        let (cmd, arg_str) = line.split_at(1);
        let arg = arg_str.parse::<u32>().unwrap();
        Cmd {
            cmd: cmd.chars().nth(0).unwrap(),
            arg,
        }
    }
}

impl State {
    fn new() -> State {
        State {
            x: 0,
            y: 0,
            dir: 90,
        }
    }

    fn angle_to_d(angle: i32) -> (i32, i32) {
        match angle {
            0 => (0, 1),    // north
            90 => (1, 0),   // east
            180 => (0, -1), // south
            270 => (-1, 0), // west
            _ => {
                panic!("invalid angle: {}", angle);
            }
        }
    }

    fn apply(self: &mut Self, cmd: &Cmd) {
        let (dx, dy) = match cmd.cmd {
            'N' => (0i32, 1i32),
            'S' => (0, -1),
            'E' => (1, 0),
            'W' => (-1, 0),
            'L' => {
                self.dir = (self.dir - cmd.arg as i32).rem_euclid(360);
                (0, 0)
            }
            'R' => {
                self.dir = (self.dir + cmd.arg as i32).rem_euclid(360);
                (0, 0)
            }
            'F' => State::angle_to_d(self.dir),
            _ => {
                panic!("eek");
            }
        };
        self.x += dx * cmd.arg as i32;
        self.y += dy * cmd.arg as i32;
    }
}

fn parse_file(lines: &str) -> Vec<Cmd> {
    lines.lines().map(|line| Cmd::parse(line)).collect()
}

fn run(cmds: &Vec<Cmd>) -> i32 {
    let mut state = State::new();
    for cmd in cmds.iter() {
        dbg!(&state);
        state.apply(cmd);
    }
    dbg!(&state);
    state.x.abs() + state.y.abs()
}

fn main() {
    let contents = std::fs::read_to_string("input/12.txt").expect("read failed");
    dbg!(&contents);
    let cmds = dbg!(parse_file(contents.as_str()));
    dbg!(run(&cmds));
}
