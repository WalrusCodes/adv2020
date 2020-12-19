use std::collections::HashMap;

#[derive(Debug)]
struct State {
    // Number mapping to the last turn it was spoken.
    last: HashMap<u32, u32>,
    last_turn: u32,
    next: u32,
}

impl State {
    fn from_starting_numbers(input: &str) -> State {
        let mut state = State {
            last: HashMap::new(),
            last_turn: 0,
            next: 0,
        };

        for x in input.split(',') {
            state
                .last
                .insert(x.parse::<u32>().unwrap(), state.last_turn + 1);
            state.last_turn += 1;
        }
        state
    }

    // Performs one turn of speaking numbers.
    fn step(self: &mut Self) {
        // next "next" - look at next in past numbers
        let next_next = if let Some(prev_turn) = self.last.get(&self.next) {
            self.last_turn + 1 - prev_turn
        } else {
            0
        };
        self.last.insert(self.next, self.last_turn + 1);
        self.last_turn += 1;
        self.next = next_next;
    }

    fn run_until_turn(self: &mut Self, last_turn: u32) {
        while self.last_turn < last_turn {
            self.step()
        }
    }
}

fn main() {
    let input = "1,20,8,12,0,14";
    let mut state = State::from_starting_numbers(input);
    dbg!(&state);
    state.run_until_turn(30000000 - 1);
    dbg!(&state.next);
}
