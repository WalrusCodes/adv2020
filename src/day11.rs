type State = Vec<Vec<char>>;

fn parse_state(lines: &str) -> State {
    lines
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

// Rules:
// 1) If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes
//    occupied.
// 2) If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat
//    becomes empty.

fn count_occupied_neighbors(state: &State, row: isize, col: isize) -> u8 {
    let mut cnt = 0;
    for r in (row - 1)..=(row + 1) {
        if r < 0 || r >= (state.len() as isize) {
            continue;
        }
        for c in (col - 1)..=(col + 1) {
            if c < 0 || c >= (state[0].len() as isize) {
                continue;
            }
            if r == row && c == col {
                continue;
            }
            if state[r as usize][c as usize] == '#' {
                cnt += 1;
            }
        }
    }
    cnt
}

// Rules for part 2:
// * count in each of 8 directions until you hit a chair
// * if occupied: 5 or more visible occupied seats -> seat becomes empty.
// * if empty: no visible occupied seats -> seat becomes occupied

fn count_los_occupied_seats(state: &State, row: isize, col: isize) -> u8 {
    let mut cnt = 0;
    for dy in -1 ..= 1 {
        for dx in -1 ..= 1 {
            if dy == 0 && dx == 0 {
                continue;
            }
            let mut i = 1;
            loop {
                let r = row + i * dy;
                let c = col + i * dx;
                if r < 0 || r >= (state.len() as isize) {
                    break;
                }
                if c < 0 || c >= (state[0].len() as isize) {
                    break;
                }
                match state[r as usize][c as usize] {
                    '#' => { cnt += 1; break; },
                    'L' => { break; },
                    _ => {},
                }
                i += 1;
            }
        }
    }
    cnt
}

// Do one step, return new state and whether anything changed.
fn step_v2(state: &State) -> (State, bool) {
    let mut out = state.clone();
    let mut changed = false;
    for row in 0..state.len() {
        for col in 0..state[0].len() {
            let cnt = count_los_occupied_seats(state, row as isize, col as isize);
            match state[row][col] {
                'L' if cnt == 0 => {
                    out[row][col] = '#';
                    changed = true;
                }
                '#' if cnt >= 5 => {
                    out[row][col] = 'L';
                    changed = true;
                }
                _ => {}
            }
        }
    }
    (out, changed)
}

fn print_state(state: &State) {
    println!("");
    for row in state.iter() {
        println!("{}", row.iter().cloned().collect::<String>());
    }
}

fn run(initial_state: &State) -> usize {
    let mut state = initial_state.clone();
    loop {
        let result = step_v2(&state);
        state = result.0;
        if !result.1 { 
            break;
        }
    }
    print_state(&state);
    state.iter().fold(0, |cnt, line| line.iter().filter(|&&x| x == '#').count() + cnt)
}

fn main() {
    let contents = std::fs::read_to_string("input/11.txt").expect("read failed");
    let initial_state = parse_state(contents.as_str());
    print_state(&initial_state);
    dbg!(run(&initial_state));
}
