use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Default, Clone)]
struct State {
    points: HashSet<Point>,
    cycle: u32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }

    fn neighbors(self: &Self) -> [Point; 26] {
        let mut out = [Point::default(); 26];
        let mut idx = 0;
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if x == 0 && y == 0 && z == 0 {
                        continue;
                    }
                    out[idx].x = self.x + x;
                    out[idx].y = self.y + y;
                    out[idx].z = self.z + z;
                    idx += 1;
                }
            }
        }
        out
    }
}

impl State {
    fn parse(input: &str) -> State {
        let mut points = HashSet::new();
        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    points.insert(Point::new(x as i32, y as i32, 0));
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
            for nb_pt in pt.neighbors().iter() {
                if let Some(count) = out.get_mut(nb_pt) {
                    *count += 1;
                } else {
                    out.insert(nb_pt.clone(), 1);
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

fn part1(state: &State) -> u32 {
    let mut state = state.clone();
    for _ in 0..6 {
        state = state.step();
    }
    state.count_active()
}

fn main() {
    let contents = std::fs::read_to_string("input/17.txt").expect("read failed");
    let state = State::parse(&contents);
    dbg!(part1(&state));
}
