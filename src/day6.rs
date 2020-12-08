use std::collections::HashSet;

fn line_to_set(line: &str) -> HashSet<char> {
    line.chars().collect()
}

// Find a union of all characters in given lines.
fn questions_union(s: &str) -> HashSet<char> {
    s.lines().fold(HashSet::new(), |set, line| {
        set.union(&line_to_set(line)).cloned().collect()
    })
}

// Find an intersection of all characters in given lines.
fn questions_intersection(s: &str) -> HashSet<char> {
    let mut lines = s.lines();

    let mut union = HashSet::new();
    union.extend(line_to_set(lines.next().unwrap()));
    lines.fold(union, |set, line| {
        set.intersection(&line_to_set(line)).cloned().collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn questions_intersection_works() {
        assert_eq!(questions_intersection("ab\nac"), line_to_set("a"));
    }
}

fn main() {
    let contents = std::fs::read_to_string("input/6.txt").expect("read failed");
    let groups = contents.split("\n\n").collect::<Vec<&str>>();
    let union_sum: usize = groups.iter().map(|x| questions_union(x).len()).sum();
    dbg!(union_sum);
    let intersection_sum: usize = groups.iter().map(|x| questions_intersection(x).len()).sum();
    dbg!(intersection_sum);
}
