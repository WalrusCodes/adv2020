use std::time::{Duration, Instant};

type Time = u64;
type BusId = u32;

fn parse_input(input: &str) -> (Time, Vec<BusId>) {
    let lines = input.lines().collect::<Vec<&str>>();
    assert_eq!(lines.len(), 2);
    let start_time = lines[0].parse::<Time>().unwrap();
    let bus_ids = lines[1]
        .split(',')
        .filter(|&x| x != "x")
        .map(|x| x.parse::<BusId>().unwrap())
        .collect();
    (start_time, bus_ids)
}

fn parse_input_v2(input: &str) -> Vec<(Time, BusId)> {
    let lines = input.lines().collect::<Vec<&str>>();
    assert_eq!(lines.len(), 2);
    lines[1]
        .split(',')
        .enumerate()
        .filter(|&(_, x)| x != "x")
        .map(|(i, x)| (i as Time, x.parse::<BusId>().unwrap()))
        .collect()
}

// Returns time we needed to wait and bus id.
fn find_earliest_bus(start_time: Time, buses: &[BusId]) -> (Time, BusId) {
    let mut time = start_time;
    loop {
        for &bus in buses.iter() {
            if ((time as BusId) % bus) == 0 {
                return (time - start_time, bus);
            }
        }
        time += 1;
    }
}

fn find_magic_timestamp_brute_force1(bus_offsets: &[(Time, BusId)]) -> Time {
    let mut time : Time = 0;
    let start = Instant::now();
    loop {
        let mut ok = true;
        for &(offset, bus_id) in bus_offsets.iter() {
            if (time + offset) % (bus_id as Time) != 0 {
                ok = false;
                break;
            }
        }
        if ok {
            return time;
        }

        // 1e8 in 500ms
        // 1e14
        //
        // 500ms * 1e6 = 0.5M seconds
        // we know that t will be > ~100'000'000'000'000
        //
        // with "996" optimization: 500ms * 1e3 = 500s

        time += 1;
        if time > 100_000_000 {
            let delta = Instant::now() - start;
            panic!("we're done: {:?}", delta);
        }
    }
}

fn find_magic_timestamp(bus_offsets: &[(Time, BusId)]) -> Time {
    let mut offset = 0;
    let mut step = 1;
    for &(bus_offset, bus_id) in bus_offsets.iter() {
        while (offset + bus_offset) % (bus_id as u64) != 0 {
            offset += step;
        }
        step *= bus_id as u64;
    }
    offset
}

// [1] t % 3 == 0
// [2] (t + 1) % 5 == 0
// [3] (t + 2) % 7 == 0
//
// [1] X        X        X        X       X        X        X        X        X             X
// [2]             X              X             X              X              X             X
// [3]                X                   X                    X                    X       X
//     0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 .... 54
//
// between [1] and [2]:  9 + (3*5)   * x1
// between [2] and [3]: 19 + (  5*7) * x2
//
// between [1], [2], and [3]: ?

fn main() {
    let contents = std::fs::read_to_string("input/13.txt").expect("read failed");
    // part 1:
    // let (start_time, buses) = dbg!(parse_input(&contents));
    // let (wait_time, bus_to_take) = dbg!(find_earliest_bus(start_time, buses.as_slice()));
    // dbg!(wait_time * bus_to_take);

    // part 2:
    let bus_offsets = parse_input_v2(&contents);
    // let bus_offsets = dbg!(parse_input_v2("\n17,x,13,19"));
    // let bus_offsets = dbg!(parse_input_v2("\n3,5,7"));
    let start = Instant::now();
    let ts = find_magic_timestamp(&bus_offsets);
    let end = Instant::now();
    dbg!(ts);
    dbg!(end - start);
}
