use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

#[derive(Debug, Default, Clone)]
struct State {
    points: HashSet<Point>,
    cycle: u32,
}

struct Neighbors {
    base: Point,
    offset: Point,
    done: bool,
}

impl Neighbors {
    fn new(base: Point) -> Neighbors {
        Neighbors {
            base,
            offset: Point::new(-1, -1, -1, -1),
            done: false,
        }
    }

    fn inc(&mut self) {
        self.offset.x += 1;
        if self.offset.x > 1 {
            self.offset.x = -1;
            self.offset.y += 1;
        }
        if self.offset.y > 1 {
            self.offset.y = -1;
            self.offset.z += 1;
        }
        if self.offset.z > 1 {
            self.offset.z = -1;
            self.offset.w += 1;
        }
        if self.offset.w > 1 {
            self.done = true;
        }
    }
}

impl Iterator for Neighbors {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        if self.done {
            return None;
        }
        // Skip over all zeros, since that's us, not a neighbor.
        if self.offset.is_zero() {
            self.inc();
        }
        let out = self.base.add(&self.offset);
        self.inc();
        Some(out)
    }
}

impl Point {
    fn new(x: i32, y: i32, z: i32, w: i32) -> Point {
        Point { x, y, z, w }
    }

    fn is_zero(self: &Self) -> bool {
        self.x == 0 && self.y == 0 && self.z == 0 && self.w == 0
    }

    fn add(self: &Self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }

    fn neighbors(self: &Self) -> Neighbors {
        Neighbors::new(*self)
    }
}

impl State {
    fn parse(input: &str) -> State {
        let mut points = HashSet::new();
        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    points.insert(Point::new(x as i32, y as i32, 0, 0));
                }
            }
        }
        State { points, cycle: 0 }
    }

    fn count_active(self: &Self) -> u32 {
        self.points.len() as u32
    }

    fn tag_neighbors(self: &Self) -> HashMap<Point, u8> {
        let mut out = HashMap::<Point, u8>::new();
        for pt in self.points.iter() {
            for nb_pt in pt.neighbors() {
                if let Some(count) = out.get_mut(&nb_pt) {
                    *count += 1;
                } else {
                    out.insert(nb_pt, 1);
                }
            }
        }
        out
    }

    fn step(self: &Self) -> State {
        let nb_counts = self.tag_neighbors();
        // dbg!(&nb_counts);

        let mut new_state = State::default();
        new_state.cycle = self.cycle + 1;

        for (pt, cnt) in nb_counts.into_iter() {
            // a) active && 2 or 3 neighbors are active => active
            // b) inactive && 3 neighbors are active => active
            if cnt == 2 || cnt == 3 {
                let was_active = self.points.contains(&pt);
                if was_active || cnt == 3 {
                    new_state.points.insert(pt);
                }
            }
        }
        // dbg!(&new_state);
        new_state
    }
}

fn run(state: &State) -> u32 {
    let mut state = state.clone();
    for _ in 0..6 {
        state = state.step();
    }
    state.count_active()
}

fn main() {
    let contents = std::fs::read_to_string("input/17.txt").expect("read failed");
    let state = State::parse(&contents);
    dbg!(run(&state));
}
