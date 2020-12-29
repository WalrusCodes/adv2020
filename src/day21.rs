#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::{HashMap, HashSet};

type Ingredient<'a> = &'a str;
type Allergen<'a> = &'a str;

#[derive(Debug)]
struct InputLine<'a> {
    ingredients: HashSet<Ingredient<'a>>,
    allergens: Vec<Allergen<'a>>,
}

#[derive(Debug)]
struct State<'a> {
    input: Vec<InputLine<'a>>,

    // Map from allergens to ingredients that they can potentially apply to.
    als: HashMap<Allergen<'a>, HashSet<Ingredient<'a>>>,

    // Ingredients that contain allergens, and which allergen they contain.
    ingredients_with_als: HashMap<Ingredient<'a>, Allergen<'a>>,
}

impl<'a> State<'a> {
    fn parse(input: &str) -> State {
        fn parse_line(input: &str) -> Option<InputLine> {
            // mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
            lazy_static! {
                static ref RE: Regex =
                    Regex::new(r"^(?P<ingredients>[^(]+) \(contains (?P<allergens>.*)\)$").unwrap();
            }
            let caps = RE.captures(input)?;
            let ingredients: HashSet<Ingredient> =
                caps.name("ingredients")?.as_str().split(' ').collect();
            let allergens: Vec<Allergen> = caps.name("allergens")?.as_str().split(", ").collect();
            Some(InputLine {
                ingredients,
                allergens,
            })
        }

        let input = input
            .lines()
            .map(|line| parse_line(line).unwrap())
            .collect();
        State {
            input,
            als: HashMap::new(),
            ingredients_with_als: HashMap::new(),
        }
    }

    fn find_allergen_with_one_food(self: &mut Self) -> Option<(Allergen<'a>, Ingredient<'a>)> {
        for (al, ing_set) in self.als.iter() {
            if ing_set.len() == 1 {
                return Some((al, ing_set.iter().next().unwrap()));
            }
        }
        None
    }

    fn remove_ingredient(self: &mut Self, ing: Ingredient) {
        for ing_set in self.als.values_mut() {
            ing_set.remove(ing);
        }
    }

    fn make_allergen_map(self: &mut Self) {
        for input_line in self.input.iter() {
            for al in input_line.allergens.iter() {
                if let Some(ing_set) = self.als.get_mut(al) {
                    *ing_set = ing_set
                        .intersection(&input_line.ingredients)
                        .cloned()
                        .collect();
                } else {
                    self.als.insert(al, input_line.ingredients.clone());
                }
            }
        }
    }

    fn find_all_ingredients_with_allergens(self: &mut Self) {
        // Keep removing allergens from the list when it's clear what food they apply to.
        while let Some((al, ing)) = self.find_allergen_with_one_food() {
            self.remove_ingredient(ing);
            self.ingredients_with_als.insert(ing, al);
        }
        dbg!(&self.ingredients_with_als);
    }

    fn count_ingredients_without_known_allergens(self: &mut Self) -> usize {
        // Now count ingredients without known allergens.
        let mut cnt = 0;
        for input_line in self.input.iter() {
            for ing in input_line.ingredients.iter() {
                if !self.ingredients_with_als.contains_key(ing) {
                    cnt += 1;
                }
            }
        }
        cnt
    }

    fn part1(self: &mut Self) -> usize {
        self.make_allergen_map();
        self.find_all_ingredients_with_allergens();
        self.count_ingredients_without_known_allergens()
    }

    fn part2(self: &mut Self) -> String {
        let mut by_allergen: Vec<(Allergen, Ingredient)> = self
            .ingredients_with_als
            .iter()
            .map(|(&k, &v)| (v, k))
            .collect();
        by_allergen.sort();
        by_allergen
            .into_iter()
            .map(|(_, ing)| ing)
            .collect::<Vec<&str>>()
            .join(",")
    }
}

fn main() {
    let contents = std::fs::read_to_string("input/21.txt").expect("read failed");
    let mut state = State::parse(&contents);
    dbg!(state.part1());
    dbg!(state.part2());
}
