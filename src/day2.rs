// Represents one parsed line from input file.
#[derive(Debug)]
struct Line {
    len_from: u32,
    len_to: u32,
    ch: char,
    password: String,
}

// Parses a single line into a Line struct.
//
// Example input:
// 3-5 f: fgfff
// ^^^ ^  ^^^^^
fn parse_line(line: &str) -> Line {
    let mut items = line.split_whitespace();
    let lens = items.next().unwrap(); // 3-5
    let ch = items.next().unwrap(); // f:
    let password = items.next().unwrap(); // fgfff

    let mut lens_split = lens.split('-').map(|x| x.parse::<u32>().unwrap());
    let len_from = lens_split.next().unwrap();
    let len_to = lens_split.next().unwrap();

    Line {
        len_from,
        len_to,
        ch: ch.chars().next().unwrap(),
        password: String::from(password),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_works() {
        let line = parse_line("3-5 f: fgfff");
        assert_eq!(line.len_from, 3);
        assert_eq!(line.len_to, 5);
        assert_eq!(line.ch, 'f');
        assert_eq!(line.password, "fgfff");
    }
}

fn main() {
    let input = std::env::args().nth(1).expect("no arg");
    let contents = std::fs::read_to_string(input).expect("read failed");

    let items = contents
        .lines()
        .map(|s| parse_line(s))
        .collect::<Vec<Line>>();

    println!("{:?}", items[0]);

    // println!("find 2: {}", find2(&items));
    // println!("find 3: {}", find3(&items));
}
