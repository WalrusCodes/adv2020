use std::collections::LinkedList;

#[derive(Debug)]
struct State {
    decks: [LinkedList<u8>; 2],
}

impl State {
    fn parse(input: &str) -> State {
        let mut parsed = input.split("\n\n").map(State::parse_deck);
        State {
            decks: [parsed.next().unwrap(), parsed.next().unwrap()],
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

    fn play_round(self: &mut Self) {
        let card1 = self.decks[0].pop_front().unwrap();
        let card2 = self.decks[1].pop_front().unwrap();
        if card1 > card2 {
            self.decks[0].push_back(card1);
            self.decks[0].push_back(card2);
        } else {
            self.decks[1].push_back(card2);
            self.decks[1].push_back(card1);
        }
    }

    fn play(self: &mut Self) -> u64 {
        let mut cnt = 0;
        while !self.decks[0].is_empty() && !self.decks[1].is_empty() {
            cnt += 1;
            self.play_round();
        }
        dbg!(cnt);
        let winner_deck = if self.decks[0].is_empty() {
            &self.decks[1]
        } else {
            &self.decks[0]
        };
        winner_deck
            .iter()
            .rev()
            .enumerate()
            .fold(0, |sum, (i, &card)| sum + (i + 1) as u64 * (card as u64))
    }
}

fn main() {
    let contents = std::fs::read_to_string("input/22.txt").expect("read failed");
    let mut state = State::parse(&contents);
    dbg!(&state);
    let product = state.play();
    dbg!(&state, &product);
}
