use std::collections::HashSet;

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

// Takes a list of tiles, returns number that are black in the end.
fn part1(tiles: &[Tile]) -> usize {
    let mut black_tiles = HashSet::<Tile>::new();
    for tile in tiles.iter() {
        if black_tiles.contains(tile) {
            black_tiles.remove(tile);
        } else {
            black_tiles.insert(tile.clone());
        }
    }
    black_tiles.len()
}

fn main() {
    let contents = std::fs::read_to_string("input/24.txt").expect("read failed");
    let tiles = contents
        .lines()
        .map(parse_and_resolve)
        .collect::<Vec<Tile>>();
    // dbg!(&tiles);
    println!("black tiles: {}", part1(&tiles));
}
