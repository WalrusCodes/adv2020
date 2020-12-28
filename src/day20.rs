#![allow(dead_code)]

use std::collections::HashMap;

mod lib;

// Width (and height) of each tile.
const SIZE: usize = 10;

// One row/side of each tile.
type Line = u16;

#[derive(PartialEq, Eq, Debug, Clone, Hash, Default)]
struct Tile {
    id: usize,
    rows: Vec<Line>,
    flipped: bool,
    rotated: u32,
}

fn bit_to_char(input: bool) -> char {
    if input {
        '#'
    } else {
        '.'
    }
}

// Converts a single line to a "#.#.#...." string.
fn line_to_string(input: Line) -> String {
    (0..SIZE)
        .rev()
        .map(|i| bit_to_char(((input >> i) & 1) == 1))
        .collect()
}

fn line_to_string_u128(input: u128, size: usize) -> String {
    (0..size)
        .rev()
        .map(|i| bit_to_char(((input >> i) & 1) == 1))
        .collect()
}

fn invert(input: Line) -> Line {
    (0..SIZE).fold(0, |out, i| (out << 1) | ((input >> i) & 1))
}

fn parse_line(line: &str) -> u128 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_to_string() {
        assert_eq!(line_to_string(1), ".........#");
        assert_eq!(line_to_string(391), ".##....###");
    }

    #[test]
    fn test_invert() {
        assert_eq!(invert(1), 512);
        assert_eq!(invert(512), 1);
        assert_eq!(invert(2), 256);
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

impl Tile {
    fn parse(input: &str) -> Tile {
        let mut lines = input.lines();
        let id = Tile::parse_id(lines.next().unwrap());
        let rows = lines.map(|line| parse_line(line) as Line).collect();
        Tile {
            id,
            rows,
            ..Tile::default()
        }
    }

    fn parse_id(line: &str) -> usize {
        assert!(line.starts_with("Tile "));
        assert!(line.ends_with(':'));
        line[5..SIZE - 1].parse().unwrap()
    }

    fn north_edge(self: &Self) -> Line {
        self.rows[0]
    }

    fn west_edge(self: &Self) -> Line {
        self.rotate_cw().rows[0]
    }

    fn east_edge(self: &Self) -> Line {
        self.rotate_cw().rotate_cw().rotate_cw().rows[0]
    }

    fn south_edge(self: &Self) -> Line {
        self.rotate_cw().rotate_cw().rows[0]
    }

    fn get_edges(self: &Self) -> [Line; 4] {
        let rot1 = self.rotate_cw();
        let rot2 = rot1.rotate_cw();
        let rot3 = rot2.rotate_cw();
        [
            self.north_edge(),
            rot1.north_edge(),
            rot2.north_edge(),
            rot3.north_edge(),
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
        Tile {
            id: self.id,
            rows,
            flipped: self.flipped,
            rotated: (self.rotated + 1) % 4,
            // links: [self.links[3], self.links[0], self.links[1], self.links[2]],
        }
    }

    fn mirror_vertical(self: &Self) -> Tile {
        Tile {
            id: self.id,
            rows: self.rows.iter().cloned().rev().collect(),
            flipped: !self.flipped,
            rotated: 0,
            // links: [self.links[2], self.links[1], self.links[0], self.links[3]],
        }
    }

    // Builds all 8 variants of rotations + flippity.
    fn make_variants(self: &Self) -> [Self; 8] {
        let rot1 = self.rotate_cw();
        let rot2 = rot1.rotate_cw();
        let rot3 = rot2.rotate_cw();
        let flip0 = self.mirror_vertical();
        let flip1 = flip0.rotate_cw();
        let flip2 = flip1.rotate_cw();
        let flip3 = flip2.rotate_cw();
        [self.clone(), rot1, rot2, rot3, flip0, flip1, flip2, flip3]
    }
}

// All tiles and various helper structs.
#[derive(Debug)]
struct TileBag {
    // Mapping from tile id to tile. These tiles will get rotated once we start linking them
    // together.
    tiles: HashMap<usize, Tile>,

    // Mapping from edges to a list of Tiles, rotated such that the top edge is used as the key.
    // Each Tile here repeats 8 times with all of its variants.
    edges: HashMap<Line, Vec<Tile>>,

    // Once we perform assembly, this is where we store the tiles, first rows, then columns.
    assembled: Vec<Vec<Tile>>,
}

struct MergedTiles {
    rows: Vec<u128>,
}

impl TileBag {
    fn parse(input: &str) -> TileBag {
        let tiles = input
            .split("\n\n")
            .map(|tile_lines| {
                let t = Tile::parse(tile_lines);
                (t.id, t)
            })
            .collect();
        let mut out = TileBag {
            tiles,
            edges: HashMap::new(),
            assembled: Vec::new(),
        };
        out.build_edges();
        out
    }

    fn build_edges(self: &mut Self) {
        for t in self.tiles.values() {
            for tt in t.make_variants().iter() {
                self.edges
                    .entry(tt.north_edge())
                    .or_insert_with(|| Vec::new())
                    .push(tt.clone());
            }
        }
    }

    // Counts how many entries we have in edge map ignoring given id.
    fn count_edges(self: &Self, edge: &Line, id_to_ignore: usize) -> usize {
        let mut cnt = 0;
        for other_edge in self.edges.get(edge).unwrap() {
            // Don't count ourselves.
            if other_edge.id != id_to_ignore {
                cnt += 1;
            }
        }
        cnt
    }

    // Calculates how many other tile edges this given tile can link to.
    fn linked_tiles(self: &Self, tile: &Tile) -> usize {
        let mut cnt = 0;
        for edge in tile.get_edges().iter() {
            cnt += self.count_edges(edge, tile.id);
        }
        cnt
    }

    // Finds corner tiles - tiles with only two other tiles linked.
    fn find_corners(self: &Self) -> Vec<Tile> {
        let corners = self
            .tiles
            .values()
            .filter_map(|t| {
                if self.linked_tiles(t) == 2 {
                    Some(t.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<Tile>>();
        assert_eq!(corners.len(), 4);
        corners
    }

    fn orient_starting_tile(self: &Self, tile: &Tile) -> Tile {
        for t in tile.make_variants().iter() {
            if self.count_edges(&t.north_edge(), tile.id) == 0
                && self.count_edges(&t.west_edge(), tile.id) == 0
            {
                return t.clone();
            }
        }
        panic!();
    }

    fn get_tile_for_edge(self: &Self, edge: Line, id_to_ignore: usize) -> Option<Tile> {
        let edge_tiles = self.edges.get(&edge).unwrap();
        if edge_tiles.len() != 2 {
            return None;
        }
        for tile in edge_tiles.iter() {
            if tile.id != id_to_ignore {
                return Some(tile.clone());
            }
        }
        panic!("shouldn't get here");
    }

    // Finds a tile that matches given east edge, ignoring given tile (so to not match ourselves).
    fn find_east_neighbor(self: &Self, tile: &Tile) -> Option<Tile> {
        Some(
            self.get_tile_for_edge(invert(tile.east_edge()), tile.id)?
                .rotate_cw()
                .rotate_cw()
                .rotate_cw(),
        )
    }

    // Finds a tile that matches given south edge, ignoring given tile (so to not match ourselves).
    fn find_south_neighbor(self: &Self, tile: &Tile) -> Option<Tile> {
        self.get_tile_for_edge(invert(tile.south_edge()), tile.id)
    }

    // Fills in .assembled with all tiles, rotating/flipping them as needed.
    fn assemble(self: &mut Self) {
        // Pick one of the corner tiles to start with. Doesn't matter which, so we'll pick the last
        // one. Rotate the tile so that it is in top-left corner of the assembled picture (only
        // east and south links are used).
        let mut tile = self.orient_starting_tile(&self.find_corners().pop().unwrap());
        loop {
            self.assembled.push(Vec::new());
            loop {
                // println!("{}\n", tile);
                self.assembled.last_mut().unwrap().push(tile.clone());

                match self.find_east_neighbor(&tile) {
                    Some(t) => {
                        tile = t;
                    }
                    None => {
                        break;
                    }
                }
            }
            // Go to next row. Find the south neighbor of the first tile from previous row.
            match self.find_south_neighbor(&self.assembled.last().unwrap()[0]) {
                Some(t) => {
                    tile = t;
                }
                None => {
                    break;
                }
            }
        }
    }

    // Takes self.assembled and turns it into a giant quilt.
    fn merge(self: &mut Self) -> MergedTiles {
        const SIZE_INNER: usize = SIZE - 2;
        let quilt_side_tiles = self.assembled.len();
        let mut out: Vec<u128> = Vec::new();
        for _ in 0..(self.assembled.len() * SIZE_INNER) {
            out.push(0);
        }
        for (tile_row_idx, tile_row) in self.assembled.iter().enumerate() {
            for (tile_col_idx, tile) in tile_row.iter().enumerate() {
                for (tile_row2_idx, row) in tile.rows[1..tile.rows.len() - 1].iter().enumerate() {
                    let out_row = tile_row_idx * SIZE_INNER + tile_row2_idx;
                    // dbg!(&out_row);
                    let out_shift = (quilt_side_tiles - tile_col_idx - 1) * SIZE_INNER;
                    // dbg!(&out_shift);
                    out[out_row] |= (((*row as u128) >> 1) & 0xff) << out_shift;
                }
            }
        }
        MergedTiles { rows: out }
    }
}

type MonsterPattern = [u128; 3];
const MONSTER_WIDTH: usize = 20;
fn make_sea_monster_pattern() -> MonsterPattern {
    [
        parse_line("..................#."),
        parse_line("#....##....##....###"),
        parse_line(".#..#..#..#..#..#..."),
    ]
}

impl MergedTiles {
    // Counts number of sea monsters and tiles without sea monsters on them.
    fn count_sea_monsters(self: &mut Self, monster: &MonsterPattern) -> (usize, usize) {
        let mut cnt = 0;
        for r in 0..(self.rows.len() - monster.len()) {
            for c in 0..(self.rows.len() - MONSTER_WIDTH) {
                if self.is_sea_monster_at(monster, r, c) {
                    cnt += 1;
                    self.remove_sea_monster_at(monster, r, c);
                }
            }
        }
        let mut other = 0;
        if cnt > 0 {
            for r in self.rows.iter() {
                let mut i: u128 = *r;
                while i > 0 {
                    if (i & 1) == 1 {
                        other += 1;
                    }
                    i >>= 1;
                }
            }
        }
        (cnt, other)
    }

    fn remove_sea_monster_at(self: &mut Self, monster: &MonsterPattern, row: usize, col: usize) {
        for r in 0..monster.len() {
            for c in 0..MONSTER_WIDTH {
                if (monster[r] >> (MONSTER_WIDTH - c - 1)) & 1 == 1 {
                    self.rows[row + r] &= !(1 << (c + col));
                }
            }
        }
    }

    fn is_sea_monster_at(self: &Self, monster: &MonsterPattern, row: usize, col: usize) -> bool {
        for r in 0..monster.len() {
            for c in 0..MONSTER_WIDTH {
                if ((monster[r] >> (MONSTER_WIDTH - c - 1)) & 1 == 1)
                    && ((self.rows[row + r] >> (c + col)) & 1 == 0)
                {
                    return false;
                }
            }
        }
        true
    }

    fn rotate_cw(self: &Self) -> MergedTiles {
        let mut rows: Vec<u128> = Vec::new();
        for i in 0..self.rows.len() {
            let mut line = 0;
            for j in 0..self.rows.len() {
                line = (line << 1)
                    | ((self.rows[self.rows.len() - j - 1] >> (self.rows.len() - i - 1)) & 1);
            }
            rows.push(line);
        }
        MergedTiles { rows }
    }

    fn mirror_vertical(self: &Self) -> MergedTiles {
        MergedTiles {
            rows: self.rows.iter().cloned().rev().collect(),
        }
    }
}

fn main() {
    let contents = std::fs::read_to_string("input/20.txt").expect("read failed");
    let mut bag = TileBag::parse(&contents);

    // Part 1: find corners, multiply their ids together.
    let corners = bag.find_corners();
    // dbg!(&corners);
    let product = corners.iter().fold(1, |p, tile| p * tile.id);
    dbg!(product);

    // Part 2: reassembly & monster finding.
    bag.assemble();
    let mut merged = bag.merge();
    let monster = make_sea_monster_pattern();
    for i in 0..8 {
        let (sea_monsters, other_tiles) = merged.count_sea_monsters(&monster);
        if sea_monsters > 0 {
            dbg!(sea_monsters, other_tiles);
            break;
        }
        merged = merged.rotate_cw();
        if i == 4 {
            merged = merged.mirror_vertical();
        }
    }
}
