// Represents one parsed line from input file.
#[derive(Debug)]
struct Line {
    len_from: u32,
    len_to: u32,
    ch: char,
    password: String,
}

impl Line {
    // Parses a single line into a Line struct.
    //
    // Example input:
    // 3-5 f: fgfff
    // ^^^ ^  ^^^^^
    fn parse(line: &str) -> Option<Line> {
        let mut items = line.split_whitespace();
        let lens = items.next()?; // 3-5
        let ch = items.next()?; // f:
        let password = items.next()?; // fgfff

        let mut lens_split = lens.split('-').map(|x| x.parse::<u32>());
        let len_from = lens_split.next()?.ok()?;
        let len_to = lens_split.next()?.ok()?;

        Some(Line {
            len_from,
            len_to,
            ch: ch.chars().next()?,
            password: String::from(password),
        })
    }

    // Checks whether the password in Line matches the pattern.
    //
    // 3-5 f: fgfff
    fn validate(self: &Self) -> bool {
        let mut cnt = 0;
        for c in self.password.chars() {
            if c == self.ch {
                cnt += 1;
            }
        }
        (cnt >= self.len_from) && (cnt <= self.len_to)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_works() {
        let line = Line::parse("3-5 f: fgfff").unwrap();
        assert_eq!(line.len_from, 3);
        assert_eq!(line.len_to, 5);
        assert_eq!(line.ch, 'f');
        assert_eq!(line.password, "fgfff");
    }

    #[test]
    fn validate_line_good() {
        assert!(Line::parse("1-3 a: abcde").unwrap().validate());
    }

    #[test]
    fn validate_line_bad() {
        assert!(!Line::parse("1-3 b: cdefg").unwrap().validate());
    }
}

fn main() {
    let input = std::env::args().nth(1).expect("no arg");
    let contents = std::fs::read_to_string(input).expect("read failed");

    let items = contents
        .lines()
        .map(|s| Line::parse(s).unwrap())
        .collect::<Vec<Line>>();

    let valid_passwords = items
        .iter()
        .fold(0, |cnt, l| if l.validate() { cnt + 1 } else { cnt });

    println!("valid passwords: {}", valid_passwords);
}
