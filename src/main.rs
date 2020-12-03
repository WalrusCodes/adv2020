use std::collections::HashSet;

// Day 1, part 1:
//
// Given a set of numbers, finds 2 numbers a, b, such that a+b==2020, and
// returns a*b.
fn find2(items: &HashSet<i32>) -> i32 {
    for a in items.iter() {
        let b = 2020 - a;
        if items.contains(&b) {
            println!("{} {}", a, b);
            return a * b;
        }
    }
    panic!("not found!");
}

// Day 1, part 2:
//
// Given a set of numbers, finds 3 numbers a, b, c, such that a+b+c==2020, and
// returns a*b*c.
fn find3(items: &HashSet<i32>) -> i32 {
    for a in items.iter() {
        for b in items.iter() {
            let c = 2020 - a - b;
            if items.contains(&c) {
                println!("{} {} {}", a, b, c);
                return a * b * c;
            }
        }
    }
    panic!("not found!");
}

fn main() {
    let input = std::env::args().nth(1).expect("no arg");
    let contents = std::fs::read_to_string(input).expect("read failed");

    let items = contents
        .lines()
        .map(|s| s.parse::<i32>().expect("failed to parse"))
        .collect::<HashSet<i32>>();

    println!("find 2: {}", find2(&items));
    println!("find 3: {}", find3(&items));
}
