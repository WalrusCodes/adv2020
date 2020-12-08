#[macro_use]
extern crate lazy_static;

use std::collections::{HashMap, HashSet};

use regex::Regex;

#[derive(Debug)]
struct BagAndCount {
    name: String,
    count: u32,
}

#[derive(Debug)]
struct Contents {
    bags: Vec<BagAndCount>,
}

#[derive(Debug)]
struct Rules {
    rules: HashMap<String, Contents>,
}

impl Contents {
    // Parses one line from rules.
    //
    // Turns this:
    // faded yellow bags contain 4 mirrored fuchsia bags, 4 dotted indigo bags, 3 faded orange bags, 5 plaid crimson bags.
    // Into this:
    // ("faded yellow", Contents{BagAndCount{name="mirrored fuchsia", count=4} ...})
    fn parse(line: &str) -> (String, Contents) {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\w+ \w+) bags contain (.*)\.$").unwrap();
            static ref RE2: Regex = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
        }
        let captures = RE.captures(line).expect("failed to parse line");
        let name = String::from(&captures[1]);
        // let contents_str = String::from(&captures[2]);

        let mut bags = vec![];
        for capture in RE2.captures_iter(&captures[2]) {
            let count = capture[1].parse::<u32>().unwrap();
            let name = String::from(&capture[2]);
            bags.push(BagAndCount { name, count });
        }

        (name, Contents { bags })
    }
}

impl Rules {
    // Parses the whole rules file.
    fn parse(lines: &str) -> Rules {
        let mut rules = Rules {
            rules: HashMap::new(),
        };
        for line in lines.lines() {
            let (name, contents) = Contents::parse(line);
            rules.rules.insert(name, contents);
        }
        rules
    }
}

// Create a mapping from a bag name to all bags that can directly contain it.
fn create_map(rules: &Rules) -> HashMap<String, HashSet<String>> {
    let mut bag_to_parents: HashMap<String, HashSet<String>> = HashMap::new();
    for (parent, contents) in rules.rules.iter() {
        for c in contents.bags.iter() {
            if !bag_to_parents.contains_key(&c.name) {
                bag_to_parents.insert(c.name.clone(), HashSet::new());
            }
            bag_to_parents
                .get_mut(&c.name)
                .unwrap()
                .insert(parent.clone());
        }
    }
    bag_to_parents
}

fn get_outer_bags(map: &HashMap<String, HashSet<String>>, bag: &str) -> HashSet<String> {
    // "shiny gold" -> set of parents
    // set of parents -> superset of parents
    let mut parents = map[bag].clone();
    println!("parents of {:?}: {:?}", bag, parents);
    let mut queue = parents.clone();
    while !queue.is_empty() {
        let p = queue.iter().nth(0).unwrap().clone();
        queue.remove(&p);
        println!("queue={:?} p={:?}", queue, p);
        if !map.contains_key(&p) {
            continue;
        }
        for pp in map.get(&p).unwrap() {
            if !parents.contains(pp) {
                println!("adding {:?} to parents & queue", pp);
                parents.insert(pp.clone());
                queue.insert(pp.clone());
            }
        }
    }

    parents
}

fn calculate_inner_bag_counts(rules: &Rules, bag: &str, counts: &mut HashMap<String, u32>) -> u32 {
    if counts.contains_key(bag) {
        return counts[bag];
    }
    let mut cnt = 1;

    let rule = rules.rules.get(bag);
    if rule.is_some() {
        for bag_and_count in rule.unwrap().bags.iter() {
            cnt += bag_and_count.count
                * calculate_inner_bag_counts(rules, bag_and_count.name.as_str(), counts);
        }
    }

    counts.insert(String::from(bag), cnt);

    cnt
}

fn main() {
    let contents = std::fs::read_to_string("input/7.txt").expect("read failed");
    let rules = Rules::parse(&contents);
    // dbg!(rules);
    let map = create_map(&rules);
    let outer_bags = get_outer_bags(&map, "shiny gold");
    dbg!(&outer_bags);
    dbg!(outer_bags.len());

    let mut counts = HashMap::new();
    dbg!(calculate_inner_bag_counts(&rules, "shiny gold", &mut counts) - 1);
}
