#[derive(Debug)]
struct State {
    cups: [u8; 9],
}

impl State {
    fn build(input: u32) -> State {
        let mut inp = input;
        let mut cups = [0u8; 9];
        for i in (0..9).rev() {
            cups[i] = (inp % 10) as u8;
            inp /= 10;
        }

        State { cups }
    }

    fn calculate_dest_cup(self: &mut Self) -> u8 {
        let mut dest_cup = self.cups[0] - 1;
        loop {
            if dest_cup == 0 {
                dest_cup = 9;
            } else if self.cups[1..4].contains(&dest_cup) {
                dest_cup -= 1;
            } else {
                break;
            }
        }
        dest_cup
    }

    fn do_move(self: &mut Self) {
        let dest_cup = self.calculate_dest_cup();
        dbg!(&dest_cup);
        let dest_cup_idx = self.cups.iter().position(|&x| x == dest_cup).unwrap();
        dbg!(&dest_cup_idx);

        let mut new_cups = [0u8; 9];
        let mut new_idx = 0;

        for i in 4..dest_cup_idx {
            new_cups[new_idx] = self.cups[i];
            new_idx += 1;
        }
        new_cups[new_idx] = dest_cup;
        new_idx += 1;
        for i in 1..4 {
            new_cups[new_idx] = self.cups[i];
            new_idx += 1;
        }
        for i in (dest_cup_idx + 1)..9 {
            new_cups[new_idx] = self.cups[i];
            new_idx += 1;
        }
        new_cups[new_idx] = self.cups[0];
        self.cups = new_cups;

        // From: <starting><3 cups that we move><XXXXX><dest cup><YYYYY>
        //
        // To:   <XXXXX><dest cup><3 cups that we move><YYYYY><starting>
    }
}

fn main() {
    // let simple_input = 389125467;
    let input = 219347865;
    let mut state = State::build(input);
    dbg!(&state);
    for _ in 0..100 {
        state.do_move();
    }
    dbg!(&state);
}
