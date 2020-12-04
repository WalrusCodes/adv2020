use std::collections::HashMap;

// Describes one document (passport?).
#[derive(Debug, Clone)]
struct Record {
    // Maps from field name, e.g., "byr" to its value.
    fields: HashMap<String, String>,
}

impl Record {
    // Checks that all required fields are present.
    fn is_valid(self: &Self, required_fields: &Vec<&str>) -> bool {
        for field in required_fields.iter() {
            if !self.fields.contains_key(*field) {
                return false;
            }
        }
        true
    }

    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    // hgt (Height) - a number followed by either cm or in:

    //     If cm, the number must be at least 150 and at most 193.
    //     If in, the number must be at least 59 and at most 76.

    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    //
    // iyr:2010 ecl:gry hgt:181cm
    // pid:591597745 byr:1920 hcl:#6b5442 eyr:2029 cid:123
    fn is_valid_advanced(self: &Self) -> Option<()> {
        let byr = self.fields.get("byr")?.parse::<i32>().ok()?;
        if byr < 1920 || byr > 2002 {
            return None;
        }

        let iyr = self.fields.get("iyr")?.parse::<i32>().ok()?;
        if iyr < 2010 || iyr > 2020 {
            return None;
        }

        let eyr = self.fields.get("eyr")?.parse::<i32>().ok()?;
        if eyr < 2020 || eyr > 2030 {
            return None;
        }

        let hgt_str = &self.fields.get("hgt")?.as_str();
        if hgt_str.len() < 3 {
            return None;
        }
        let (min, max) = match &hgt_str[hgt_str.len() - 2..] {
            "in" => (59, 76),
            "cm" => (150, 193),
            _ => {
                return None;
            }
        };
        let hgt = hgt_str[0..hgt_str.len() - 2].parse::<i32>().ok()?;
        if hgt < min || hgt > max {
            return None;
        }

        let hcl = self.fields.get("hcl")?.as_str();
        if hcl.len() != 7 || hcl.chars().nth(0)? != '#' {
            return None;
        }
        for c in hcl.chars().skip(1) {
            if !((c >= '0' && c <= '9') || (c >= 'a' && c <= 'f')) {
                return None;
            }
        }

        let ecl = self.fields.get("ecl")?.as_str();
        match ecl  {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {},
            _ => { return None; }
        }

        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        let pid = self.fields.get("pid")?.as_str();
        if pid.len() != 9 {
            return None;
        }
        for c in pid.chars() {
            if c < '0' || c > '9' {
                return None;
            }
        }

        Some(())
    }
}

fn text_to_records(inp: &str) -> Vec<Record> {
    let mut records = vec![];
    let mut record = Record {
        fields: HashMap::new(),
    };
    for line in inp.lines() {
        if line.len() == 0 {
            records.push(record.clone());
            record.fields.clear();
            continue;
        }
        // line:
        // hgt:177cm hcl:#602927 iyr:2016 pid:404183620
        for item in line.split_whitespace() {
            // item:
            // hgt:177cm
            let split_field = item.split(':').collect::<Vec<&str>>();
            assert_eq!(split_field.len(), 2);
            record
                .fields
                .insert(String::from(split_field[0]), String::from(split_field[1]));
        }
    }
    if record.fields.len() > 0 {
        records.push(record);
    }
    records
}

fn count_valid_records(records: &Vec<Record>, required_fields: &Vec<&str>) -> u32 {
    records.iter().fold(0, |cnt, r| {
        cnt + if r.is_valid(required_fields) { 1 } else { 0 }
    })
}

fn count_valid_records_advanced(records: &Vec<Record>) -> u32 {
    records.iter().fold(0, |cnt, r| {
        cnt + if r.is_valid_advanced().is_some() {
            1
        } else {
            0
        }
    })
}

fn main() {
    let contents = std::fs::read_to_string("input/4_1.txt").expect("read failed");

    // byr (Birth Year)
    // iyr (Issue Year)
    // eyr (Expiration Year)
    // hgt (Height)
    // hcl (Hair Color)
    // ecl (Eye Color)
    // pid (Passport ID)
    // cid (Country ID) - optional
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let records = text_to_records(contents.as_str());
    dbg!(records.len());
    // dbg!(&records[0], &records[1]);

    dbg!(count_valid_records(&records, &required_fields));
    dbg!(count_valid_records_advanced(&records));
}
