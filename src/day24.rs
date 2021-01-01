use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Dir {
    E,
    Se,
    Sw,
    W,
    Nw,
    Ne,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Tile {
    row: i32,
    col: i32,
}

// Iterator that turns a string into Dir directions.
struct TileParser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> TileParser<'a> {
    fn from(input: &str) -> TileParser {
        TileParser { input, pos: 0 }
    }

    fn get_char(self: &Self, i: usize) -> Option<char> {
        self.input.chars().nth(i)
    }
}

impl<'a> Iterator for TileParser<'a> {
    type Item = Dir;

    fn next(&mut self) -> Option<Dir> {
        if self.pos >= self.input.len() {
            return None;
        }
        let c = self.get_char(self.pos).unwrap();
        let c2 = self.get_char(self.pos + 1);

        let (inc, out) = match (c, c2) {
            ('e', _) => (1, Dir::E),
            ('w', _) => (1, Dir::W),
            ('n', Some('w')) => (2, Dir::Nw),
            ('n', Some('e')) => (2, Dir::Ne),
            ('s', Some('w')) => (2, Dir::Sw),
            ('s', Some('e')) => (2, Dir::Se),
            _ => {
                panic!("invalid input: {}", self.input);
            }
        };

        self.pos += inc;
        Some(out)
    }
}

// Parses a tile, figures out where it ends up, produces a Tile.
fn parse_and_resolve(input: &str) -> Tile {
    let mut row = 0i32;
    let mut col = 0i32;

    // Directions: e, se, sw, w, nw, and ne
    // Reference tile: XX
    //
    //         0    1    2    3    4
    // 0  ..|    |    |    |    |    |
    // 1  ....|    | nw | ne |     |     |
    // 2  ..|    | w  | XX | e  |    |
    // 3  ....|    | sw | se |     |     |
    // 4  ..|    |    |    |    |    |
    // 5  ....|    |    |    |     |     |
    // 6  ..|    |    | nw | ne |    |
    // 7  ....|    | w  | XX | e   |     |
    // 8  ..|    |    | sw | se |    |
    //
    //      (row, col)
    // Ref: (2, 2)
    //
    // e    (2, 3)
    // se   (3, 2)  <- col stays the same
    // sw   (3, 1)
    // w    (2, 1)
    // nw   (1, 1)
    // ne   (1, 2)  <- col stays the same
    //
    // Ref: (7, 2)
    //
    // e    (7, 3)
    // se   (8, 3)
    // sw   (8, 2)  <- col stays the same
    // w    (7, 1)
    // nw   (6, 2)  <- col stays the same
    // ne   (6, 3)
    //
    // * in case of {n,s}x{e,w}:
    //   * going west => col -= 1
    //   * going east => col stays the same
    //   * if we were on an odd row, col += 1
    //
    // Neighbors:
    // * if even row: [(0, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0)]
    // * if odd row: [(0, 1), (1, 1), (1, 0), (0, -1), (-1, 0), (-1, 1)]

    for dir in TileParser::from(input) {
        match dir {
            Dir::E => {
                col += 1;
            }
            Dir::W => {
                col -= 1;
            }
            _ => {
                if (row % 2) != 0 {
                    col += 1;
                }
                match dir {
                    Dir::Sw | Dir::Se => {
                        row += 1;
                    }
                    Dir::Nw | Dir::Ne => {
                        row -= 1;
                    }
                    _ => {}
                }
                match dir {
                    Dir::Sw | Dir::Nw => {
                        col -= 1;
                    }
                    _ => {}
                }
            }
        }
    }

    Tile { row, col }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_resolve() {
        let tile = parse_and_resolve("nwwswee");
        assert_eq!(tile.row, 0);
        assert_eq!(tile.col, 0);
    }
}

// Set of black tiles.
type State = HashSet<Tile>;

// Takes a list of tiles, returns tiles that are black.
fn part1(tiles: &[Tile]) -> State {
    let mut black_tiles = State::new();
    for tile in tiles.iter() {
        if black_tiles.contains(tile) {
            black_tiles.remove(tile);
        } else {
            black_tiles.insert(tile.clone());
        }
    }
    black_tiles
}

impl Tile {
    fn neighbors(&self) -> [(i32, i32); 6] {
        if (self.row % 2) == 0 {
            [(0, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0)]
        } else {
            [(0, 1), (1, 1), (1, 0), (0, -1), (-1, 0), (-1, 1)]
        }
    }
}

// Performs tile flips for one day in part 2.
fn do_step(state: &State) -> State {
    let mut neighbor_count: HashMap<Tile, usize> = HashMap::new();
    for tile in state {
        for (offset_row, offset_col) in tile.neighbors().iter() {
            let nt = Tile {
                row: tile.row + offset_row,
                col: tile.col + offset_col,
            };
            *neighbor_count.entry(nt).or_insert(0) += 1;
        }
    }
    // dbg!(&neighbor_count);

    let mut new_state = State::new();
    for (tile, count) in neighbor_count.into_iter() {
        // Rules:
        // * black tile && (0 or 3+ neighbors) => white
        // * white tile && 2 neighbors => black
        //
        // 0 => white
        // 1 && black => black
        // 2 => black
        // 3 => white
        if (count == 2) || ((count == 1) && state.contains(&tile)) {
            new_state.insert(tile);
        }
    }

    new_state
}

fn part2(state: &State, days: usize) -> usize {
    let mut s = state.clone();
    for i in 1..=days {
        s = do_step(&s);
        // dbg!(i, s.len());
    }
    s.len()
}

fn main() {
    let contents = std::fs::read_to_string("input/24.txt").expect("read failed");
    let tiles = contents
        .lines()
        .map(parse_and_resolve)
        .collect::<Vec<Tile>>();
    let black_tiles = part1(&tiles);
    println!("black tiles for part 1: {}", black_tiles.len());
    println!("black tiles for part 2: {}", part2(&black_tiles, 100));
}
