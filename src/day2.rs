// Represents one parsed line from input file.
#[derive(Debug)]
struct Line {
    num1: u32,
    num2: u32,
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
        let num1 = lens_split.next()?.ok()?;
        let num2 = lens_split.next()?.ok()?;

        Some(Line {
            num1,
            num2,
            ch: ch.chars().next()?,
            password: String::from(password),
        })
    }

    // Checks whether the password in Line matches the pattern for the first half of the puzzle.
    fn validate(self: &Self) -> bool {
        let mut cnt = 0;
        for c in self.password.chars() {
            if c == self.ch {
                cnt += 1;
            }
        }
        (cnt >= self.num1) && (cnt <= self.num2)
    }

    // Checks whether the password in Line matches the pattern for the first half of the puzzle.
    fn validate2(self: &Self) -> bool {
        let ch1 = self.password.chars().nth(self.num1 as usize - 1).unwrap();
        let ch2 = self.password.chars().nth(self.num2 as usize - 1).unwrap();

        let v1 = ch1 == self.ch;
        let v2 = ch2 == self.ch;

        (v1 && !v2) || (!v1 && v2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_works() {
        let line = Line::parse("3-5 f: fgfff").unwrap();
        assert_eq!(line.num1, 3);
        assert_eq!(line.num2, 5);
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

    #[test]
    fn validate_line2() {
        assert!(!Line::parse("1-3 b: cdefg").unwrap().validate2());
        assert!(Line::parse("1-3 b: bdefg").unwrap().validate2());
        assert!(!Line::parse("1-3 b: bdbfg").unwrap().validate2());
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

    let valid_passwords2 = items
        .iter()
        .fold(0, |cnt, l| if l.validate2() { cnt + 1 } else { cnt });

    println!("valid passwords method 1: {}", valid_passwords);
    println!("valid passwords method 2: {}", valid_passwords2);
}
