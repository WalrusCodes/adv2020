use std::collections::HashSet;

// Parses seat representation into seat id.
//
// BFBFBBBRLL
// BFBFBBB RLL
// 1010111 100
// ^ row   ^ column
//   0..127   0..7
fn str_to_seat_id(s: &str) -> u32 {
    let (row_str, col_str) = s.split_at(7);
    let mut row = 0;
    for c in row_str.chars() {
        row = 2 * row
            + match c {
                'B' => 1,
                'F' => 0,
                _ => {
                    panic!("bad news");
                }
            };
    }
    let mut col = 0;
    for c in col_str.chars() {
        col = 2 * col
            + match c {
                'R' => 1,
                'L' => 0,
                _ => {
                    panic!("bad news");
                }
            };
    }
    row * 8 + col
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_to_seat_id_works() {
        assert_eq!(str_to_seat_id("BFFFBBFRRR"), 567);
    }
}

// Finds a missing seat id N, where seats N - 1 and N + 1 do exist.
fn find_my_seat(seats: &HashSet<u32>) -> u32 {
    // N exists, N + 1 does not exist, N + 2 exists.
    //           ^^^^^
    for id in seats.iter() {
        if !seats.contains(&(id + 1)) && seats.contains(&(id + 2)) {
            return id + 1;
        }
    }
    panic!("sad panda");
}

fn main() {
    let contents = std::fs::read_to_string("input/5.txt").expect("read failed");
    let seat_ids = contents
        .lines()
        .map(|line| str_to_seat_id(line))
        .collect::<HashSet<u32>>();
    let max_seat_id = seat_ids.iter().max();
    dbg!(max_seat_id);
    dbg!(find_my_seat(&seat_ids));
}
