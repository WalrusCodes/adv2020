// Counts 1- and 3-number gaps.
fn count_gaps(input: &[i32]) -> (u32, u32) {
    let mut gap1 = 0;
    let mut gap3 = 0;

    for i in 1..input.len() {
        let delta = input[i] - input[i - 1];
        if delta > 3 {
            panic!("nope");
        }
        if delta == 1 {
            gap1 += 1;
        } else if delta == 3 {
            gap3 += 1;
        }
    }
    (gap1, gap3)
}

fn part1(input: &[i32]) {
    let (gap1, gap3) = dbg!(count_gaps(input));
    dbg!(gap1 * gap3);
}

fn part2(input: &[i32]) {
    let mut prev = [0u64; 3];
    let mut next = [0u64; 3];
    let numbers = input
        .iter()
        .cloned()
        .collect::<std::collections::HashSet<i32>>();

    // At 0 we have exactly 1 path for getting to 0.
    prev[0] = 1;
    prev[1] = 0;
    prev[2] = 0;

    let max = numbers.iter().max().unwrap().clone() as usize;

    for i in 1..=max {
        if numbers.contains(&(i as i32)) {
            // path where we take i
            next[0] = prev[0] + prev[1] + prev[2];
        } else {
            next[0] = 0;
        }
        // paths where we don't take i
        next[1] = prev[0];
        next[2] = prev[1];
        dbg!(i, prev, next);

        prev = next;
    }
    dbg!(next);
    dbg!(next.iter().sum::<u64>());
}

fn main() {
    let contents = std::fs::read_to_string("input/10.txt").expect("read failed");
    let mut input = contents
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    input.push(0);
    input.sort();
    input.push(input.last().unwrap() + 3);
    // part1(&input.as_slice());
    part2(&input.as_slice());
}
