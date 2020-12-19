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

fn any_rule_matches(field: u32, rules: &Rules) -> bool {
    rules.iter().any(|rule| rule.check_match(field))
}

fn calculate_error_rate(tickets: &Vec<Ticket>, rules: &Rules) -> u32 {
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

fn main() {
    let contents = std::fs::read_to_string("input/16.txt").expect("read failed");
    let (rules, my_ticket, nearby_tickets) = parse_input(&contents);
    dbg!(calculate_error_rate(&nearby_tickets, &rules));
}
