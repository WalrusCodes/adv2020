use std::collections::{HashSet, LinkedList};

#[derive(Debug)]
struct State {
    decks: [LinkedList<u8>; 2],
    seen: HashSet<(Vec<u8>, Vec<u8>)>,
}

impl State {
    fn parse(input: &str) -> State {
        let mut parsed = input.split("\n\n").map(State::parse_deck);
        State {
            decks: [parsed.next().unwrap(), parsed.next().unwrap()],
            seen: HashSet::new(),
        }
    }

    fn parse_deck(input: &str) -> LinkedList<u8> {
        assert!(input.starts_with("Player "));
        input
            .lines()
            .skip(1)
            .map(|line| line.parse::<u8>().unwrap())
            .collect()
    }

    fn state_from_subdecks(self: &Self, count1: usize, count2: usize) -> State {
        State {
            decks: [
                self.decks[0].iter().take(count1).cloned().collect(),
                self.decks[1].iter().take(count2).cloned().collect(),
            ],
            seen: HashSet::new(),
        }
    }

    fn state_to_tuple(self: &Self) -> (Vec<u8>, Vec<u8>) {
        (
            self.decks[0].iter().cloned().collect(),
            self.decks[1].iter().cloned().collect(),
        )
    }

    fn play_round(self: &mut Self) {
        let card1 = self.decks[0].pop_front().unwrap();
        let card2 = self.decks[1].pop_front().unwrap();
        // Detect if we can play recursive round.
        let winner_is_1 =
            if (card1 as usize) <= self.decks[0].len() && (card2 as usize) <= self.decks[1].len() {
                // Recursive combat.
                let mut state = self.state_from_subdecks(card1 as usize, card2 as usize);
                state.play().0
            } else {
                // Regular combat.
                card1 > card2
            };
        // Put the two cards in winner's deck.
        if winner_is_1 {
            self.decks[0].push_back(card1);
            self.decks[0].push_back(card2);
        } else {
            self.decks[1].push_back(card2);
            self.decks[1].push_back(card1);
        }
    }

    // Play the game, return (did player 1 win, winner's score).
    fn play(self: &mut Self) -> (bool, u64) {
        while !self.decks[0].is_empty() && !self.decks[1].is_empty() {
            let state_tuple = self.state_to_tuple();
            if self.seen.contains(&state_tuple) {
                return (true, 0);
            }
            self.seen.insert(state_tuple);

            self.play_round();
        }
        let winner_is_1 = self.decks[1].is_empty();

        let winner_deck = &self.decks[if winner_is_1 { 0 } else { 1 }];
        let winner_score = winner_deck
            .iter()
            .rev()
            .enumerate()
            .fold(0, |sum, (i, &card)| sum + (i + 1) as u64 * (card as u64));
        (winner_is_1, winner_score)
    }
}

fn main() {
    let contents = std::fs::read_to_string("input/22.txt").expect("read failed");
    let mut state = State::parse(&contents);
    dbg!(&state);
    let (who_won, product) = state.play();
    dbg!(&state, &who_won, &product);
}
