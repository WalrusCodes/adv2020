use std::collections::{HashMap, HashSet};

type Ticket = Vec<u32>;

// Describes one Rule, e.g.:
// departure location: 27-672 or 680-954
#[derive(Debug)]
struct Rule {
    name: String,
    ranges: Vec<(u32, u32)>,
}

type Rules = Vec<Rule>;

fn parse_ticket(input: &str) -> Ticket {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

impl Rule {
    // Parses "27-672" into (27, 672).
    fn parse_range(input: &str) -> (u32, u32) {
        let parts = input.split('-').collect::<Vec<&str>>();
        assert_eq!(parts.len(), 2);
        (parts[0].parse().unwrap(), parts[1].parse().unwrap())
    }

    fn parse(input: &str) -> Rule {
        // departure location: 27-672 or 680-954
        let parts = input.split(": ").collect::<Vec<&str>>();
        assert_eq!(parts.len(), 2);

        let ranges = parts[1]
            .split(" or ")
            .map(|x| Rule::parse_range(x))
            .collect();

        Rule {
            name: String::from(parts[0]),
            ranges,
        }
    }

    fn check_match(self: &Self, value: u32) -> bool {
        for &(from, to) in self.ranges.iter() {
            if value >= from && value <= to {
                return true;
            }
        }
        false
    }
}

// Parses input into 1) rules, 2) my ticket, 3) nearby tickets.
fn parse_input(input: &str) -> (Rules, Ticket, Vec<Ticket>) {
    let groups = input.split("\n\n").collect::<Vec<&str>>();
    assert_eq!(groups.len(), 3);
    let rules = groups[0].lines().map(|x| Rule::parse(x)).collect();
    let my_ticket = parse_ticket(groups[1].lines().nth(1).unwrap());
    let nearby_tickets = groups[2].lines().skip(1).map(|x| parse_ticket(x)).collect();

    (rules, my_ticket, nearby_tickets)
}

fn any_rule_matches(field: u32, rules: &[Rule]) -> bool {
    rules.iter().any(|rule| rule.check_match(field))
}

fn is_ticket_valid(ticket: &Ticket, rules: &[Rule]) -> bool {
    !ticket.iter().any(|&field| !any_rule_matches(field, rules))
}

fn calculate_error_rate(tickets: &[Ticket], rules: &[Rule]) -> u32 {
    let mut error_rate = 0;

    for tick in tickets.iter() {
        for &field in tick.iter() {
            if !any_rule_matches(field, rules) {
                error_rate += field;
            }
        }
    }

    error_rate
}

fn find_matching_rules(tickets: &[Ticket], rules: &[Rule], field_idx: usize) -> HashSet<usize> {
    let mut out = (0..rules.len()).collect::<HashSet<usize>>();
    for tick in tickets.iter() {
        // dbg!(&out, &tick);
        let field = tick[field_idx];
        let mut to_remove = vec![];
        for &rule_idx in out.iter() {
            if !rules[rule_idx].check_match(field) {
                to_remove.push(rule_idx);
            }
        }
        for rule_idx in to_remove.iter() {
            out.remove(rule_idx);
        }
    }
    out
}

fn find_field_names(
    fields_to_matching_rules: &Vec<HashSet<usize>>,
    rules: &Rules,
) -> HashMap<String, usize> {
    let mut f_map = fields_to_matching_rules.clone();
    let mut out = HashMap::new();
    while out.len() < f_map.len() {
        dbg!(&f_map, &out);
        // 1. find an entry in f_map that has only one element (idx into rules),
        let idx = f_map
            .iter()
            .enumerate()
            .filter(|(_, rule_indices)| rule_indices.len() == 1)
            .next()
            .unwrap()
            .0;
        dbg!(&idx);
        // 2. get the rule idx, then remove it from all HashSets in f_map.
        let only_rule_idx = f_map[idx].iter().cloned().next().unwrap();
        dbg!(&only_rule_idx);
        for hs in f_map.iter_mut() {
            hs.remove(&only_rule_idx);
        }
        // 3. add the rule's name to out.
        out.insert(rules[only_rule_idx].name.clone(), idx);
    }
    out
}

fn part2_answer(names_to_fields: &HashMap<String, usize>, ticket: &Ticket) -> u64 {
    let mut out = 1u64;
    for (name, field_idx) in names_to_fields.iter() {
        if name.starts_with("departure") {
            out *= ticket[*field_idx] as u64;
        }
    }
    out
}

fn main() {
    let contents = std::fs::read_to_string("input/16.txt").expect("read failed");
    let (rules, my_ticket, nearby_tickets) = parse_input(&contents);
    let valid_tickets = nearby_tickets
        .iter()
        .filter(|t| is_ticket_valid(t, &rules))
        .cloned()
        .collect::<Vec<Ticket>>();

    dbg!(nearby_tickets.len());
    dbg!(valid_tickets.len());

    // dbg!(calculate_error_rate(&nearby_tickets, &rules));
    // dbg!(find_matching_rules(&valid_tickets, &rules, 0));

    let fields_to_matching_rules = (0..rules.len())
        .map(|field_idx| find_matching_rules(&valid_tickets, &rules, field_idx))
        .collect::<Vec<HashSet<usize>>>();
    // dbg!(&fields_to_matching_rules);
    let names_to_fields = find_field_names(&fields_to_matching_rules, &rules);
    dbg!(&names_to_fields);
    dbg!(part2_answer(&names_to_fields, &my_ticket));
}
