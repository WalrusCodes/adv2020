#![allow(dead_code)]

use std::collections::HashMap;

mod lib;

const SIZE: usize = 10;

type Line = u16;

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
struct Tile {
    id: usize,
    rows: Vec<Line>,
}

fn invert_bits(input: Line) -> Line {
    (0..SIZE).fold(0, |out, i| (out << 1) | ((input >> i) & 1))
}

fn line_to_string(input: Line) -> String {
    let mut out = String::with_capacity(SIZE);
    for i in 0..SIZE {
        out.push(if ((input >> (SIZE - i - 1)) & 1) == 1 {
            '#'
        } else {
            '.'
        });
    }
    out
}

impl Tile {
    fn parse(input: &str) -> Tile {
        let mut lines = input.lines();
        let id = Tile::parse_id(lines.next().unwrap());
        let rows = lines.map(|line| Tile::parse_line(line)).collect();
        Tile { id, rows }
    }

    fn parse_line(line: &str) -> Line {
        let mut out = 0;
        for c in line.chars() {
            out = (out << 1)
                | match c {
                    '.' => 0,
                    '#' => 1,
                    _ => {
                        panic!("invalid char: {}", c);
                    }
                };
        }
        out
    }

    fn parse_id(line: &str) -> usize {
        assert!(line.starts_with("Tile "));
        assert!(line.ends_with(':'));
        line[5..SIZE - 1].parse().unwrap()
    }

    fn top_edge(self: &Self) -> Line {
        self.rows[0]
    }

    //   0 ->
    //   xxxx 1
    // ^ xxxx |
    // | xxxx v
    // 3 xxxx
    //   <- 2
    fn get_edges(self: &Self) -> [Line; 4] {
        let rot1 = self.rotate_cw();
        let rot2 = rot1.rotate_cw();
        let rot3 = rot2.rotate_cw();
        [
            self.top_edge(),
            rot1.top_edge(),
            rot2.top_edge(),
            rot3.top_edge(),
        ]
    }

    fn rotate_cw(self: &Self) -> Tile {
        let mut rows: Vec<Line> = Vec::new();
        for i in 0..SIZE {
            let mut line = 0;
            for j in 0..SIZE {
                line = (line << 1) | ((self.rows[SIZE - j - 1] >> (SIZE - i - 1)) & 1);
            }
            rows.push(line);
        }
        Tile { id: self.id, rows }
    }

    fn mirror_vertical(self: &Self) -> Tile {
        Tile {
            id: self.id,
            rows: self.rows.iter().cloned().rev().collect(),
        }
    }

    // How many other tile edges can this tile link to?
    fn linked_tiles(self: &Self, map: &HashMap<Line, Vec<usize>>) -> usize {
        let mut cnt = 0;
        for edge in self.get_edges().iter() {
            for &other_edge_id in map.get(edge).unwrap() {
                if self.id != other_edge_id {
                    cnt += 1;
                }
            }
        }
        cnt
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for line in self.rows.iter() {
            if first {
                first = false;
            } else {
                f.write_str("\n")?;
            }
            f.write_str(&line_to_string(*line))?;
        }
        Ok(())
    }
}

// Build a mapping from edges to a list of tile ids.
fn build_map(tiles: &[Tile]) -> HashMap<Line, Vec<usize>> {
    let mut out = HashMap::new();
    for t in tiles {
        // 4 rotations.
        for &e in t.get_edges().iter() {
            out.entry(e).or_insert_with(|| Vec::new()).push(t.id);
        }
        // mirror + 4 rotations.
        for &e in t.mirror_vertical().get_edges().iter() {
            out.entry(e).or_insert_with(|| Vec::new()).push(t.id);
        }
    }
    out
}

fn main() {
    let contents = std::fs::read_to_string("input/20.txt").expect("read failed");
    let tiles = contents
        .split("\n\n")
        .map(Tile::parse)
        .collect::<Vec<Tile>>();

    // part 1
    let map = build_map(&tiles);
    let corners_it = tiles.iter().filter_map(|t| {
        if t.linked_tiles(&map) == 2 {
            Some(t.id)
        } else {
            None
        }
    });
    let product = corners_it.fold(1, |p, x| p * x);
    dbg!(product);
}
