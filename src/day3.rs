// Traverses the map, returns number of trees encountered.
fn traverse(map: &Vec<&str>, dx: usize, dy: usize) -> u32 {
    let width = map[0].len();
    let mut x = 0;
    let mut y = 0;
    let mut cnt = 0;
    while y < map.len() {
        let ch = map[y].chars().nth(x).unwrap();
        if ch == '#' {
            // tree!
            cnt += 1;
        }
        y += dy;
        x = (x + dx) % width;
    }
    cnt
}

// Traverses the map, tests out multiple slopes, returns the number of trees hit multiplied
// together.
//
// Slopes:
// Right 1, down 1.
// Right 3, down 1.
// Right 5, down 1.
// Right 7, down 1.
// Right 1, down 2.
fn traverse2(map: &Vec<&str>) -> u32 {
    let slopes : Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut result = 1;
    for (dx, dy) in slopes.iter() {
        result *= traverse(map, *dx, *dy);
    }
    result
}

fn main() {
    let contents = std::fs::read_to_string("input/3_1.txt").expect("read failed");

    let items = contents.lines().collect::<Vec<&str>>();

    // Part 1:
    let trees = traverse(&items, 3, 1);
    println!("trees: {}", trees);

    // Part 2:
    let trees2 = traverse2(&items);
    println!("trees part 2: {}", trees2);
}
