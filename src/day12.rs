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
    // +y is north
    ship_x: i32,
    ship_y: i32,
    wp_x: i32,
    wp_y: i32,
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
            ship_x: 0,
            ship_y: 0,
            wp_x: 10,
            wp_y: 1,
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
                // wp_x: 10, wp_y: 1
                //
                //     X    (-1, 10)
                //      .
                //      .
                //      .
                //      .              X   (10, 1)
                //    [ship] ..........
                assert_eq!(cmd.arg % 90, 0);
                for _ in 0..(cmd.arg / 90) {
                    let tmp = self.wp_x;
                    self.wp_x = -self.wp_y;
                    self.wp_y = tmp;
                }
                (0, 0)
            },
            'R' => {
                assert_eq!(cmd.arg % 90, 0);
                for _ in 0..(cmd.arg / 90) {
                    let tmp = self.wp_x;
                    self.wp_x = self.wp_y;
                    self.wp_y = -tmp;
                }
                (0, 0)
            },
            'F' => (self.wp_x, self.wp_y),
            _ => {
                panic!("eek");
            }
        };
        if cmd.cmd == 'F' {
            self.ship_x += dx * cmd.arg as i32;
            self.ship_y += dy * cmd.arg as i32;
        } else {
            self.wp_x += dx * cmd.arg as i32;
            self.wp_y += dy * cmd.arg as i32;
        }
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
    state.ship_x.abs() + state.ship_y.abs()
}

fn main() {
    let contents = std::fs::read_to_string("input/12.txt").expect("read failed");
    dbg!(&contents);
    let cmds = dbg!(parse_file(contents.as_str()));
    dbg!(run(&cmds));
}
