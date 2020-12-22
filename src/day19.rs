#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Rule {
    Letter(char),
    Choose(Vec<Vec<usize>>),
}

#[derive(Debug, Default)]
struct Problem {
    rules: Vec<Rule>,
    inputs: Vec<String>,
}

impl Rule {
    fn parse_numbers(s: &str) -> Vec<usize> {
        s.split(' ').map(|x| x.parse().unwrap()).collect()
    }

    fn parse(rhs: &str) -> Rule {
        if rhs.starts_with('"') {
            Rule::Letter(rhs.chars().nth(1).unwrap())
        } else {
            Rule::Choose(rhs.split(" | ").map(Self::parse_numbers).collect())
        }
    }
}

impl Problem {
    fn parse_rule_line(line: &str) -> (usize, Rule) {
        let mut parts = line.split(": ");
        let idx = parts.next().unwrap().parse().unwrap();
        let rule = Rule::parse(parts.next().unwrap());
        (idx, rule)
    }

    fn parse_rules(lines: &str) -> Vec<Rule> {
        let mut idx_rules: Vec<(usize, Rule)> = lines.lines().map(Self::parse_rule_line).collect();
        idx_rules.sort_by_key(|x| x.0);
        idx_rules.into_iter().map(|x| x.1).collect()
    }

    fn parse(lines: &str) -> Problem {
        let mut parts = lines.split("\n\n");
        let rules = Self::parse_rules(parts.next().unwrap());
        let inputs = parts.next().unwrap().lines().map(str::to_string).collect();
        Problem { rules, inputs }
    }

    fn check_apply_subpart(
        self: &Self,
        input: &str,
        rules: &[usize],
        start: usize,
        end: usize,
        cache: &mut HashMap<(usize, usize, usize), bool>,
    ) -> bool {
        if rules.len() == 1 {
            self.check_match_recursive(input, rules[0], start, end, cache)
        } else {
            for first_ends in start + 1..end {
                if self.check_match_recursive(input, rules[0], start, first_ends, cache)
                    && self.check_apply_subpart(input, &rules[1..], first_ends, end, cache)
                {
                    return true;
                }
            }
            false
        }
    }

    // Given an input string, e.g., "ababbb" and start..end indices within this string, return true
    // if we can match self.rules[rule_idx], false otherwise.
    //
    // The approach:
    // * if rule_idx points to a "Letter" rule, see if input[start..end] is a single character long
    //   and matches the expected character, fail otherwise.
    // * if rule_idx points to a "Choose", try any of the options available:
    //    * for each option available, try out all possible lengths of the remaining substring
    //      input[start..end]. "check_apply_subpart" is a recursive helper that we pass in the
    //      remaining parts of the rule.
    //
    // Since this recalculates same stuff over and over as it finds different paths through the
    // rules, we cache any results that we have calculated. A tuple of (rule_idx, start, end)
    // uniquely identifies the result for this input, so we use that as a cache key.
    fn check_match_recursive(
        self: &Self,
        input: &str,
        rule_idx: usize,
        start: usize,
        end: usize,
        cache: &mut HashMap<(usize, usize, usize), bool>,
    ) -> bool {
        let key = (rule_idx, start, end);
        if let Some(&result) = cache.get(&key) {
            return result;
        }
        let result = match &self.rules[rule_idx] {
            Rule::Letter(ch) => ((end - start) == 1) && (input.chars().nth(start).unwrap() == *ch),
            Rule::Choose(options) => {
                let mut ok = false;
                for opt in options.iter() {
                    if self.check_apply_subpart(input, opt, start, end, cache) {
                        ok = true;
                        break;
                    }
                }
                ok
            }
        };
        cache.insert(key, result);
        result
    }

    // Returns true if given word completely matches rule 0.
    fn check_match(self: &Self, input: &str) -> bool {
        // (rule_idx, start, end) -> result
        let mut cache: HashMap<(usize, usize, usize), bool> = HashMap::new();
        self.check_match_recursive(input, 0, 0, input.len(), &mut cache)
    }

    // Returns count of words from self.inputs that completely match rule 0.
    fn count_matches(self: &Self) -> usize {
        self.inputs
            .iter()
            .map(|x| if self.check_match(x) { 1 } else { 0 })
            .sum()
    }
}

fn main() {
    let contents = std::fs::read_to_string("input/19_2.txt").expect("read failed");
    let problem = Problem::parse(&contents);
    // dbg!(&problem);
    // dbg!(problem.check_match(&problem.inputs[0]));
    dbg!(problem.count_matches());
}
